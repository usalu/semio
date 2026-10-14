use crate::insert_member_action::InsertMemberAction;
use crate::remove_member_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertMemberAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member_action::RemoveMemberAction> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.member_actions.len());
    vec![Op::from(remove_member_action::RemoveMemberAction { index: at })]

    })())
}
