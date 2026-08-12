//! ↩️ `change-bridge-lane-width-m` — undo restores BASE's bridge lane width.

use super::mutation::ChangeBridgeLaneWidthM;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBridgeLaneWidthM, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeLaneWidthM(ChangeBridgeLaneWidthM { new_bridge_lane_width_m: base.bridge_lane_width_m.clone() })]
}
//#endregion 🔖️Inverse
