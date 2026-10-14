//! ↩️ `change-stock-label` inverse — reconstructs the pre-change label from BASE state; `change`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::change_stock_label::ChangeStockLabel>>(_payload: &super::ChangeStockLabel, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(super::ChangeStockLabel { new_label: base.stock_label.clone() })]

    })())
}
//#endregion 🔖️Inverse
