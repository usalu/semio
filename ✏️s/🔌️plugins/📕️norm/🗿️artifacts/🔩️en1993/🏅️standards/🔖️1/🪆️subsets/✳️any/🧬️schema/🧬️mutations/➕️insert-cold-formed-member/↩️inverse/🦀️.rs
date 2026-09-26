use super::InsertColdFormedMember;
use crate::mutations::{remove_cold_formed_member, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertColdFormedMember, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.cold_formed_members.len());
    vec![En1993Mutation::RemoveColdFormedMember(remove_cold_formed_member::RemoveColdFormedMember { index: at })]
}
