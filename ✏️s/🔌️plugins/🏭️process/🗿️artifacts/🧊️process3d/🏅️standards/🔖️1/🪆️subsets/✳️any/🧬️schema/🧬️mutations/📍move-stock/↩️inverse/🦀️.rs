//! ↩️ `move-stock` inverse — reconstructs the pre-move pose from BASE state; `move` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::move_stock::MoveStock>>(_payload: &super::MoveStock, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(super::MoveStock { new_pose: base.stock_pose.clone() })]

    })())
}
//#endregion 🔖️Inverse
