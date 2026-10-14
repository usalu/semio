//! 🔺️ Diff for `change-bridge-lane`.
use crate::change_bridge_lane::ChangeBridgeLane;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeBridgeLane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_lane == payload.new_bridge_lane {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lane: Some(payload.new_bridge_lane), ..Default::default() })
}
