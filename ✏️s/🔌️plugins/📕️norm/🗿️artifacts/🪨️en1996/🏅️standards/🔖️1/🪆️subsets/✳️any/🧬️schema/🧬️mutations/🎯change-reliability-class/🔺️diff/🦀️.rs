//! 🔺️ `change-reliability-class` diff — sets the document's reliability class; the same value is a `mutation.no-op`.

use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeReliabilityClass, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.reliability_class == payload.new_reliability_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "reliability_class already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { reliability_class: Some(payload.new_reliability_class), ..Default::default() })
}
