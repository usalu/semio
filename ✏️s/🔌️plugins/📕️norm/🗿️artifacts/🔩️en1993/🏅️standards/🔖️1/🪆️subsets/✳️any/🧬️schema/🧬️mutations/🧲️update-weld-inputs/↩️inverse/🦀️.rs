//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateWeldInputs;
use crate::mutations::remove_member_action;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateWeldInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.member_actions.iter().find(|x| x.id == payload.member_action.id) {
        vec![En1993Mutation::UpdateWeldInputs(UpdateWeldInputs { member_action: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { index: base.member_actions.len() })]
    }
}
