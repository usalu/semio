//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateColdFormedInputs;
use crate::mutations::remove_cold_formed_member;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.cold_formed_members.iter().find(|x| x.id == payload.cold_formed_member.id) {
        vec![En1993Mutation::UpdateColdFormedInputs(UpdateColdFormedInputs { cold_formed_member: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveColdFormedMember(remove_cold_formed_member::RemoveColdFormedMember { index: base.cold_formed_members.len() })]
    }
}
