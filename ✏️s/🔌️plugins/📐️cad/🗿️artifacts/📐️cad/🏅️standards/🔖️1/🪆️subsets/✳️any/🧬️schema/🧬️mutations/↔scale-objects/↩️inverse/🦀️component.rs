//! ↩️ Inverse for `ScaleObjects` — the reciprocal factor undoes a relative composed scale.
use super::mutation::ScaleObjects;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleObjects, _base: &CadSnapshot) -> Vec<CadMutation> {
    let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
    vec![CadMutation::ScaleObjects(ScaleObjects { object_ids: payload.object_ids.clone(), sx: inv(payload.sx), sy: inv(payload.sy), sz: inv(payload.sz) })]
}
//#endregion 🔖️Inverse
