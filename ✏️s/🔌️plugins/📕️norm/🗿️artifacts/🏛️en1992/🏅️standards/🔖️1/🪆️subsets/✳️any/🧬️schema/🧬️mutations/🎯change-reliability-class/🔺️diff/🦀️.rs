use semio_s_artifact_norm_en1992_diff::En1992Diff;
use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeReliabilityClass, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.reliability_class == payload.new_reliability_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Reliability class already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { reliability_class: Some(payload.new_reliability_class), ..Default::default() })
}
