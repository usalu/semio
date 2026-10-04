//! ↩️ `change-stock-label` inverse — reconstructs the pre-change label from BASE state; `change`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeStockLabel, base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Process3dMutation::ChangeStockLabel(super::ChangeStockLabel { new_label: base.stock_label.clone() })]

    })())
}
//#endregion 🔖️Inverse
