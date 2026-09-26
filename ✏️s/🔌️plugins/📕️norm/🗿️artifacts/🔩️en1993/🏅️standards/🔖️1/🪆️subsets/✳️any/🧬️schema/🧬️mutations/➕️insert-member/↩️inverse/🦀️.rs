use super::InsertMember;
use crate::mutations::{remove_member, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertMember, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.members.len());
    vec![En1993Mutation::RemoveMember(remove_member::RemoveMember { index: at })]
}
