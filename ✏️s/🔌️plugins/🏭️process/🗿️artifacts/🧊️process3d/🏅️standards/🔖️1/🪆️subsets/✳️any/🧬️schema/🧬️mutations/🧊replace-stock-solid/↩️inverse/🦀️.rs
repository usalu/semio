//! ↩️ `replace-stock-solid` inverse — reconstructs the pre-replace handle from BASE state; `replace`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceStockSolid, base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Process3dMutation::ReplaceStockSolid(super::ReplaceStockSolid { new_solid: base.stock_solid.clone() })]

    })())
}
//#endregion 🔖️Inverse
