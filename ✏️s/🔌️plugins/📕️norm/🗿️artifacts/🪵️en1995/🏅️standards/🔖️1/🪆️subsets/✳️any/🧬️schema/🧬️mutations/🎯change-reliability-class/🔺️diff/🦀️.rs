//! 🔺️ `change-reliability-class` sparse diff construction — writes only `En1995Diff.reliability_class` from the payload.

use semio_s_artifact_norm_en1995_diff::En1995Diff;
use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReliabilityClass, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.reliability_class == payload.new_reliability_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Reliability class already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { reliability_class: Some(payload.new_reliability_class), ..Default::default() })
}
//#endregion 🔖️Diff
