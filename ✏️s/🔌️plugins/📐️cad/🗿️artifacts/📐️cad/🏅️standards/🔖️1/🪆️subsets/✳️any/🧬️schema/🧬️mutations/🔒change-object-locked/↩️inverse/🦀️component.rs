//! ↩️ Inverse for `ChangeObjectLocked` — recovers the pre-mutation `locked` from `base`.
use super::mutation::ChangeObjectLocked;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeObjectLocked, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::ChangeObjectLocked(ChangeObjectLocked { pane: payload.pane, object_id: payload.object_id.clone(), new_locked: object.locked })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
