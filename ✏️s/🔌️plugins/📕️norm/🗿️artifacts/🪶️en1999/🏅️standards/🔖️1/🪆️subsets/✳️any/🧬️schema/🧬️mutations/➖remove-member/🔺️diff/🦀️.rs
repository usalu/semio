//! 🔺️ `remove-member` diff.

use crate::diff::En1999Diff;
use crate::mutations::remove_member::RemoveMember;
use crate::En1999Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !base.members.iter().any(|m| m.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member id {}", payload.id), Vec::<String>::new());
    }
    let members: Vec<_> = base.members.iter().filter(|m| m.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(En1999Diff { members: Some(members), ..Default::default() })
}
