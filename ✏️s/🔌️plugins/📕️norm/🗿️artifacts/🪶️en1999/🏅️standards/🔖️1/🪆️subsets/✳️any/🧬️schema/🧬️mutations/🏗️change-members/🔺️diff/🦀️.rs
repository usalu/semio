//! 🔺️ `change-members` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_members::ChangeMembers;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeMembers, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.members == &payload.members {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(payload.members.clone()), ..Default::default() })
}
