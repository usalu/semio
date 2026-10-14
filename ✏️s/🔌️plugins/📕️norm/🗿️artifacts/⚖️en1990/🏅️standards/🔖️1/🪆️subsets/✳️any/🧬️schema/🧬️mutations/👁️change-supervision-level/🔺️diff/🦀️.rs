//! 🔺️ `change-supervision-level` sparse diff.

use crate::change_supervision_level::ChangeSupervisionLevel;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.supervision_level == &mutation.new_supervision_level {
        return MutationOutcome::empty().warning("mutation.no-op", "supervision_level already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        supervision_level: Some(mutation.new_supervision_level.clone()),
        ..En1990Diff::default()
    })
}
