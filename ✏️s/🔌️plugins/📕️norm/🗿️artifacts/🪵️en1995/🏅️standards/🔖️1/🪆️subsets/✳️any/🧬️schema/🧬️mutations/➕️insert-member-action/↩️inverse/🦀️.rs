use crate::insert_member_action::InsertMemberAction;
use crate::remove_member_action;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertMemberAction, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member_action::RemoveMemberAction> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    let at = payload.index.min(item.actions.len());
    vec![Op::from(remove_member_action::RemoveMemberAction { member_id: payload.member_id.clone(), index: at })]

    })())
}
