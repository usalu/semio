//! 🔺️ `change-members` sparse diff.

use super::ChangeMembers;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeMembers, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.members == &mutation.new_members {
        return MutationOutcome::empty().warn("mutation.no-op", "members already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        members: Some(mutation.new_members.clone()),
        ..En1990Diff::default()
    })
}
