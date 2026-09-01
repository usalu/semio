//! 🔺️ `change-bridge-lane` — sparse diff construction.

use super::ChangeBridgeLane;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeLane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_lane == payload.new_bridge_lane {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bridge lane already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lane: Some(payload.new_bridge_lane.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
