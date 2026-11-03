//! 🔺️ `change-reliability-class` — sparse diff construction.

use crate::change_reliability_class::ChangeReliabilityClass;
use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReliabilityClass, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.reliability_class == payload.new_reliability_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Reliability class already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { reliability_class: Some(payload.new_reliability_class), ..Default::default() })
}
//#endregion 🔖️Diff
