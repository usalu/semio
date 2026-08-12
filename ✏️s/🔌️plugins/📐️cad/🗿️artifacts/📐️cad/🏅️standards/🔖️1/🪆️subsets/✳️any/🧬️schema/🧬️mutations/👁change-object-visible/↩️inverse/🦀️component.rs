//! ↩️ Inverse for `ChangeObjectVisible` — recovers the pre-mutation `visible` from `base`.
use super::mutation::ChangeObjectVisible;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeObjectVisible, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::ChangeObjectVisible(ChangeObjectVisible { pane: payload.pane, object_id: payload.object_id.clone(), new_visible: object.visible })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
