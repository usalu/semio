use super::RemoveColdFormedMember;
use crate::mutations::{insert_cold_formed_member, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveColdFormedMember, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.cold_formed_members.len() { return Vec::new(); }
    vec![En1993Mutation::InsertColdFormedMember(insert_cold_formed_member::InsertColdFormedMember { index: payload.index, cold_formed_member: base.cold_formed_members[payload.index].clone() })]
}
