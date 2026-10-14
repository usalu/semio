use crate::remove_member_action::RemoveMemberAction;
use crate::insert_member_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveMemberAction, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member_action::InsertMemberAction> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    if payload.index >= item.actions.len() { return Vec::new(); }
    vec![Op::from(insert_member_action::InsertMemberAction { member_id: payload.member_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })]

    })())
}
