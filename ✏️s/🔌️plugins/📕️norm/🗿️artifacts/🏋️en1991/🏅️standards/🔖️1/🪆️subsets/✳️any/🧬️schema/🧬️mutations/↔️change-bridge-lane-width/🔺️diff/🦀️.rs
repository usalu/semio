//! Diff for `change-bridge-lane-width`.
use super::ChangeBridgeLaneWidth;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeBridgeLaneWidth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_lane_width == payload.new_bridge_lane_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lane_width: Some(payload.new_bridge_lane_width), ..Default::default() })
}
