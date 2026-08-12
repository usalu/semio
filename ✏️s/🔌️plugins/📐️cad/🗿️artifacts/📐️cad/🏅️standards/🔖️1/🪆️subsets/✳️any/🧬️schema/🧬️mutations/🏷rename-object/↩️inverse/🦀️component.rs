//! ↩️ Inverse for `RenameObject` — recovers the pre-mutation `label` from `base`.
use super::mutation::RenameObject;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RenameObject, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::RenameObject(RenameObject { pane: payload.pane, object_id: payload.object_id.clone(), new_label: object.label.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
