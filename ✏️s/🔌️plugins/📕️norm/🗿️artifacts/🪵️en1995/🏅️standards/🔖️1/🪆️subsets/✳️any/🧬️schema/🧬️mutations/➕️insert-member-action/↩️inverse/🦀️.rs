use crate::insert_member_action::InsertMemberAction;
use crate::remove_member_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertMemberAction, base: &En1995Snapshot) -> Option<crate::remove_member_action::RemoveMemberAction> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return None; };
    let at = payload.index.min(item.actions.len());
    Some(remove_member_action::RemoveMemberAction { member_id: payload.member_id.clone(), index: at })
}
