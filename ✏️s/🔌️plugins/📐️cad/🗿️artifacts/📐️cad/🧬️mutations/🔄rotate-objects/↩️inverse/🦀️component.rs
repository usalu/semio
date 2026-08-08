//! ↩️ Inverse for `RotateObjects`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;

//#region 🔖️Inverse
pub fn inverse(_base: &CadProjection, object_ids: &[String], ax: f64, ay: f64, az: f64, angle: f64) -> Vec<CadMutation> {
    vec![CadMutation::RotateObjects { object_ids: object_ids.to_vec(), ax, ay, az, angle: -angle }]
}
//#endregion 🔖️Inverse
