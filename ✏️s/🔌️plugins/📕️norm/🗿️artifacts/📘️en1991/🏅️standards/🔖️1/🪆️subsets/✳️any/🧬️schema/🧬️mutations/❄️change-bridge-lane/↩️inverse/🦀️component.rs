//! ↩️ `change-bridge-lane` — undo restores BASE's bridge lane count.

use super::mutation::ChangeBridgeLane;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBridgeLane, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeLane(ChangeBridgeLane { new_bridge_lane: base.bridge_lane.clone() })]
}
//#endregion 🔖️Inverse
