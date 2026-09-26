//! ↩️ `change-members` inverse.

use super::ChangeMembers;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeMembers, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeMembers(ChangeMembers { new_members: base.members.clone() })]
}
