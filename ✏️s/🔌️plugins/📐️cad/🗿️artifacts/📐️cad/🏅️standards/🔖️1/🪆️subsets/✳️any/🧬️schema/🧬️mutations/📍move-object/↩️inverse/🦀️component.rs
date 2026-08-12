//! ↩️ Inverse for `MoveObject` — recovers the pre-mutation `origin` from `base`.
use super::mutation::MoveObject;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &MoveObject, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::MoveObject(MoveObject { pane: payload.pane, object_id: payload.object_id.clone(), new_origin: object.origin })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
