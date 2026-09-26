use super::RemoveMemberAction;
use crate::mutations::{insert_member_action, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &RemoveMemberAction, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    if payload.index >= item.actions.len() { return Vec::new(); }
    vec![En1995Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { member_id: payload.member_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })]
}
