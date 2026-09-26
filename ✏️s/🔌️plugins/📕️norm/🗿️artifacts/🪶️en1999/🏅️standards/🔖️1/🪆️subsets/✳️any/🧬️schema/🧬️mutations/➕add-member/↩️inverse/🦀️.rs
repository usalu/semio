//! ↩️ `add-member` inverse.

use crate::mutations::add_member::AddMember;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;
use crate::mutations::remove_member;

pub fn inverse(payload: &AddMember, _base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::RemoveMember(remove_member::RemoveMember { id: payload.member.id.clone() })]
}
