//! ↩️ Inverse for `DragObjects` — the negated offset undoes a relative drag regardless of `base`.
use super::mutation::DragObjects;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DragObjects, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DragObjects(DragObjects { object_ids: payload.object_ids.clone(), dx: -payload.dx, dy: -payload.dy, dz: -payload.dz })]
}
//#endregion 🔖️Inverse
