//! ↩️ `replace-stock-solid` inverse — reconstructs the pre-replace handle from BASE state; `replace`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_stock_solid::ReplaceStockSolid>>(_payload: &super::ReplaceStockSolid, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(super::ReplaceStockSolid { new_solid: base.stock_solid.clone() })]

    })())
}
//#endregion 🔖️Inverse
