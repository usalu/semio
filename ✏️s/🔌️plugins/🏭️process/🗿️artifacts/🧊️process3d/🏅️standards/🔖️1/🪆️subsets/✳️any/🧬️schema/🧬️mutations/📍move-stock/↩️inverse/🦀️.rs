//! ↩️ `move-stock` inverse — reconstructs the pre-move pose from BASE state; `move` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::MoveStock, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::MoveStock(super::MoveStock { new_pose: base.stock_pose.clone() })]
}
//#endregion 🔖️Inverse
