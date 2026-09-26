use super::InsertMemberAction;
use crate::mutations::{remove_member_action, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertMemberAction, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.member_actions.len());
    vec![En1993Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { index: at })]
}
