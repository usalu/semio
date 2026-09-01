//! ↩️ `replace-stock-solid` inverse — reconstructs the pre-replace handle from BASE state; `replace`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceStockSolid, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::ReplaceStockSolid(super::ReplaceStockSolid { new_solid: base.stock_solid.clone() })]
}
//#endregion 🔖️Inverse
