use super::InsertMember;
use crate::mutations::{remove_member, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &InsertMember, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let at = payload.index.min(base.members.len());
    vec![En1995Mutation::RemoveMember(remove_member::RemoveMember { index: at })]
}
