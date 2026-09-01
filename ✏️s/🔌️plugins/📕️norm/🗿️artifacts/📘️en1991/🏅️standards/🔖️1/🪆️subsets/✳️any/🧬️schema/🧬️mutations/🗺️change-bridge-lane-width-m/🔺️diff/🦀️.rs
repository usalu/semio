//! 🔺️ `change-bridge-lane-width-m` — sparse diff construction.

use super::ChangeBridgeLaneWidthM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeLaneWidthM, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_bridge_lane_width_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bridge lane width m must be a finite number.", Vec::<String>::new());
    }
    if base.bridge_lane_width_m == payload.new_bridge_lane_width_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bridge lane width m already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lane_width_m: Some(payload.new_bridge_lane_width_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
