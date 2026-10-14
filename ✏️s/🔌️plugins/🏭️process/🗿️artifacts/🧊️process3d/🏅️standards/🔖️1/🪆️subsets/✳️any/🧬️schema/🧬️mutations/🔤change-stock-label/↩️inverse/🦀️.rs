//! ↩️ `change-stock-label` inverse — reconstructs the pre-change label from BASE state; `change`
//! is its own inverse partner (per `📓️taxonomy.md`).

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_stock_label::ChangeStockLabel>>(_payload: &super::ChangeStockLabel, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(super::ChangeStockLabel { new_label: base.stock_label.clone() })]

    })())
}
//#endregion 🔖️Inverse
