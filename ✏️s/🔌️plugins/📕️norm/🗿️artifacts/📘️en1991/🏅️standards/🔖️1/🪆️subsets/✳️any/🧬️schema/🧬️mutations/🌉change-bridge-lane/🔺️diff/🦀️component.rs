//! 🔺️ `change-bridge-lane` — sparse diff construction.

use super::mutation::ChangeBridgeLane;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeLane, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { bridge_lane: Some(payload.new_bridge_lane.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
