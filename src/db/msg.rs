
use crate::{
    base::{paging_data::Paging, user_info::UserInfo, pg_client::PGClient},
    data_models::notice::{NoticeType, NoticeComment},
    errors::MyError,
};

/// 获取通知
pub async fn get_comments<'a>(
    user: &UserInfo,
    client: &PGClient,
    paging: &Paging<'a>,
) -> Result<Vec<NoticeComment>, MyError> {
    let _stmt = include_str!("../../sql/msg/get_comments.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
    // 未读消息，需要设置 read: true
    let mut unread_vec = vec![];

    let vec = client
    .query(&stmt, &[
        NoticeType::Comment.to_i16(),
        &user.id, 
        paging.limit(), 
        paging.offset()
    ])
    .await?
    .iter()
    .map(|row| {
        let notice = NoticeComment::from(row);
        if !notice.read {
            unread_vec.push(notice.id);
        }
        notice
    })
    .collect::<Vec<NoticeComment>>();

    Ok(vec)
}


pub async fn send_notice(
    sender: &i32, 
    notice_type: &i16, 
    sender_obj_id: &String, 
    addressee_id: &i32,
    client: &PGClient,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/msg/insert_notices.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;

    client
        .query(&stmt, &[sender, notice_type, sender_obj_id, addressee_id])
        .await?
        .iter()
        .map(|_| ())
        .collect::<Vec<()>>()
        .pop()
        .ok_or(MyError::InternalServerError)
}