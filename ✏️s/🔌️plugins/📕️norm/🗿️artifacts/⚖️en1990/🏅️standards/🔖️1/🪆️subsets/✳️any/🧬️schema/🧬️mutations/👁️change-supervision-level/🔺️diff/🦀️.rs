//! 🔺️ `change-supervision-level` sparse diff.

use super::ChangeSupervisionLevel;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.supervision_level == &mutation.new_supervision_level {
        return MutationOutcome::empty().warn("mutation.no-op", "supervision_level already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        supervision_level: Some(mutation.new_supervision_level.clone()),
        ..En1990Diff::default()
    })
}
