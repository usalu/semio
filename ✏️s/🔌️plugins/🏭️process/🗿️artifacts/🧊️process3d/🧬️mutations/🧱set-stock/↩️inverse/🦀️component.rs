//! ↩️ Inverse for `SetStock`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dDocument, Stock};

//#region 🔖️Inverse
pub fn inverse(base: &Process3dDocument, _stock: &Stock) -> Vec<Process3dMutation> {
    vec![Process3dMutation::SetStock { stock: base.stock.clone() }]
}
//#endregion 🔖️Inverse
