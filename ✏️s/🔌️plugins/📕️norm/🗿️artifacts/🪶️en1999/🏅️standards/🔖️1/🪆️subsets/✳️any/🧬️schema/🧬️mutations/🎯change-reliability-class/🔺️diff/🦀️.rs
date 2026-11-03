//! 🔺️ `change-reliability-class` diff.

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeReliabilityClass, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.reliability_class == payload.new_reliability_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Reliability class already set.");
    }
    protocol::MutationOutcome::new(En1999Diff { reliability_class: Some(payload.new_reliability_class), ..Default::default() })
}
