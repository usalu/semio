use crate::remove_member_action::RemoveMemberAction;
use crate::insert_member_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveMemberAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member_action::InsertMemberAction> {
    Ok((|| {
    if payload.index >= base.member_actions.len() { return Vec::new(); }
    vec![Op::from(insert_member_action::InsertMemberAction { index: Some(payload.index), member_action: base.member_actions[payload.index].clone() })]

    })())
}
