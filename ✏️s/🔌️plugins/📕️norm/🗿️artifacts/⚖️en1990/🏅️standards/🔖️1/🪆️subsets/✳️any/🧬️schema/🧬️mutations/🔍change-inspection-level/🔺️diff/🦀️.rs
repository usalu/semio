//! 🔺️ `change-inspection-level` sparse diff.

use crate::change_inspection_level::ChangeInspectionLevel;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeInspectionLevel, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.inspection_level == &mutation.new_inspection_level {
        return MutationOutcome::empty().warning("mutation.no-op", "inspection_level already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        inspection_level: Some(mutation.new_inspection_level.clone()),
        ..En1990Diff::default()
    })
}
