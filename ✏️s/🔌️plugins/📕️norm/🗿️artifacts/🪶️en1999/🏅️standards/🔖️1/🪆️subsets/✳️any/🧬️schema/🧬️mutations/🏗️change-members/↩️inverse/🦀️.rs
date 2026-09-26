//! ↩️ `change-members` inverse.

use crate::mutations::change_members::ChangeMembers;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeMembers, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeMembers(ChangeMembers { members: base.members.clone() })]
}
