//! 🔺️ `change-reliability-class` sparse diff.

use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeReliabilityClass, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.reliability_class == mutation.new_reliability_class {
        return MutationOutcome::empty().warning("mutation.no-op", "reliability_class already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        reliability_class: Some(mutation.new_reliability_class),
        ..En1990Diff::default()
    })
}
