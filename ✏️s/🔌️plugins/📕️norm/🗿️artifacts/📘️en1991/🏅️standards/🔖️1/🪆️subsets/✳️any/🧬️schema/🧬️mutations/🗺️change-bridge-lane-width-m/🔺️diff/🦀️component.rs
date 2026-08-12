//! 🔺️ `change-bridge-lane-width-m` — sparse diff construction.

use super::mutation::ChangeBridgeLaneWidthM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeLaneWidthM, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { bridge_lane_width_m: Some(payload.new_bridge_lane_width_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
