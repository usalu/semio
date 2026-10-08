//! 🔺️ `remove-member` diff.

use crate::mutations::remove_member::RemoveMember;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MembersRows};

pub fn diff(payload: &RemoveMember, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !base.members.iter().any(|m| m.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member id {}", payload.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
