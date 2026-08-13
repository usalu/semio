//! ↩️ `change-stock-label` inverse — reconstructs the pre-change label from BASE state; `change`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStockLabel, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: base.stock_label.clone() })]
}
//#endregion 🔖️Inverse
