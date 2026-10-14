//! ↩️ `move-stock` inverse — reconstructs the pre-move pose from BASE state; `move` is its own
//! inverse partner (per `📓️taxonomy.md`).

use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::move_stock::MoveStock>>(_payload: &super::MoveStock, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(super::MoveStock { new_pose: base.stock_pose.clone() })]

    })())
}
//#endregion 🔖️Inverse
