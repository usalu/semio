//! 🔺️ Diff for `change-assumed-rail-sw2`.
use crate::change_assumed_rail_sw2::ChangeAssumedRailSw2;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedRailSw2, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_rail_sw2 == payload.new_assumed_rail_sw2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_rail_sw2: Some(payload.new_assumed_rail_sw2), ..Default::default() })
}
