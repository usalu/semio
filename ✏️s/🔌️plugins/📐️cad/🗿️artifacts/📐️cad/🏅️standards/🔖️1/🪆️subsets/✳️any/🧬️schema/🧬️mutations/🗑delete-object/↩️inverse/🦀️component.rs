//! ↩️ Inverse for `DeleteObject` — recreates the captured object from `base`.
use super::mutation::DeleteObject;
use crate::artifacts::cad::mutations::create_object;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteObject, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::CreateObject(create_object::mutation::CreateObject { pane: payload.pane, object: object.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
