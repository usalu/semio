//! 🔺️ `change-inspection-level` sparse diff.

use super::ChangeInspectionLevel;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeInspectionLevel, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.inspection_level == &mutation.new_inspection_level {
        return MutationOutcome::empty().warn("mutation.no-op", "inspection_level already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        inspection_level: Some(mutation.new_inspection_level.clone()),
        ..En1990Diff::default()
    })
}
