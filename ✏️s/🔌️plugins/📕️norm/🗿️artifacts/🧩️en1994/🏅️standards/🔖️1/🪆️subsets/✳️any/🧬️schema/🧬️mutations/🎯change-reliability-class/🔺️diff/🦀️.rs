//! 🔺️ Diff for `change-reliability-class`.
use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &ChangeReliabilityClass, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if false {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.reliability_class == payload.new_reliability_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { reliability_class: Some(payload.new_reliability_class), ..Default::default() })
}
