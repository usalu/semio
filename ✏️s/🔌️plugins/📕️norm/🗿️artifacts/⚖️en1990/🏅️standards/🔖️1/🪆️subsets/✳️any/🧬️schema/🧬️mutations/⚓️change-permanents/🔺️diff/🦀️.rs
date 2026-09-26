//! 🔺️ `change-permanents` sparse diff.

use super::ChangePermanents;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangePermanents, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.permanents == &mutation.new_permanents {
        return MutationOutcome::empty().warn("mutation.no-op", "permanents already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        permanents: Some(mutation.new_permanents.clone()),
        ..En1990Diff::default()
    })
}
