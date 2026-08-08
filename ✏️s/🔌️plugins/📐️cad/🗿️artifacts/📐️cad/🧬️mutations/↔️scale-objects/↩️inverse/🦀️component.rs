//! ↩️ Inverse for `ScaleObjects`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;

//#region 🔖️Inverse
pub fn inverse(_base: &CadProjection, object_ids: &[String], sx: f64, sy: f64, sz: f64) -> Vec<CadMutation> {
    let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
    vec![CadMutation::ScaleObjects { object_ids: object_ids.to_vec(), sx: inv(sx), sy: inv(sy), sz: inv(sz) }]
}
//#endregion 🔖️Inverse
