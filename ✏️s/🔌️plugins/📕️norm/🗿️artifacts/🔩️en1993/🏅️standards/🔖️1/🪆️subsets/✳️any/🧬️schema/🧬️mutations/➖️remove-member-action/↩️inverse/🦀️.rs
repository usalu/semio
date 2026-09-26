use super::RemoveMemberAction;
use crate::mutations::{insert_member_action, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveMemberAction, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.member_actions.len() { return Vec::new(); }
    vec![En1993Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { index: payload.index, member_action: base.member_actions[payload.index].clone() })]
}
