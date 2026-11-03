//! 🔺️ Diff for `change-assumed-rail-sw0`.
use crate::change_assumed_rail_sw0::ChangeAssumedRailSw0;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedRailSw0, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_rail_sw0 == payload.new_assumed_rail_sw0 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_rail_sw0: Some(payload.new_assumed_rail_sw0), ..Default::default() })
}
