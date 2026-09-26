//! ↩️ `remove-member` inverse.

use crate::mutations::remove_member::RemoveMember;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;
use crate::mutations::add_member;

pub fn inverse(payload: &RemoveMember, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    if let Some((index, member)) = base.members.iter().enumerate().find(|(_, m)| m.id == payload.id) {
        vec![En1999Mutation::AddMember(add_member::AddMember { index: index as u32, member: member.clone() })]
    } else { Vec::new() }
}
