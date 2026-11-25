use crate::remove_member_action::RemoveMemberAction;
use crate::insert_member_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveMemberAction, base: &En1995Snapshot) -> Option<crate::insert_member_action::InsertMemberAction> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    if payload.index >= item.actions.len() { return None; }
    Some(insert_member_action::InsertMemberAction { member_id: payload.member_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })
}
