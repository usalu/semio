//! ↩️ `replace-stock-solid` inverse — reconstructs the pre-replace handle from BASE state; `replace`
//! is its own inverse partner (per `📓️taxonomy.md`).

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::replace_stock_solid::ReplaceStockSolid>>(_payload: &super::ReplaceStockSolid, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(super::ReplaceStockSolid { new_solid: base.stock_solid.clone() })]

    })())
}
//#endregion 🔖️Inverse
