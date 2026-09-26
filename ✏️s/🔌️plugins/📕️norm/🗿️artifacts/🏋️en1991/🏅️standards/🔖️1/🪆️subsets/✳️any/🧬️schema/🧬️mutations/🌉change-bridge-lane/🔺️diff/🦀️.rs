//! Diff for `change-bridge-lane`.
use super::ChangeBridgeLane;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeBridgeLane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_lane == payload.new_bridge_lane {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lane: Some(payload.new_bridge_lane), ..Default::default() })
}
