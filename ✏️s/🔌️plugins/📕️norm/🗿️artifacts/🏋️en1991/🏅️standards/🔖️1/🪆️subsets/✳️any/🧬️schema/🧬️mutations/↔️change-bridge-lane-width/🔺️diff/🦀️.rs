//! 🔺️ Diff for `change-bridge-lane-width`.
use crate::change_bridge_lane_width::ChangeBridgeLaneWidth;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeBridgeLaneWidth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_lane_width == payload.new_bridge_lane_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lane_width: Some(payload.new_bridge_lane_width), ..Default::default() })
}
