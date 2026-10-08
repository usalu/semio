use super::InsertMemberAction;
use crate::mutations::{remove_member_action, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertMemberAction, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.member_actions.len());
    vec![En1993Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { index: at })]

    })())
}
