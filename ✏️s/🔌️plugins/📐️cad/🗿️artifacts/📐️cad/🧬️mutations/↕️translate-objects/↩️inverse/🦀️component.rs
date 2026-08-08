//! ↩️ Inverse for `TranslateObjects`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_base: &CadSnapshot, object_ids: &[String], dx: f64, dy: f64, dz: f64) -> Vec<CadMutation> {
    vec![CadMutation::TranslateObjects { object_ids: object_ids.to_vec(), dx: -dx, dy: -dy, dz: -dz }]
}
//#endregion 🔖️Inverse
