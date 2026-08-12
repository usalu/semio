//! ↩️ Inverse for `ChangeObjectTypology` — recovers the pre-mutation `typology` from `base`.
use super::mutation::ChangeObjectTypology;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeObjectTypology, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::ChangeObjectTypology(ChangeObjectTypology { pane: payload.pane, object_id: payload.object_id.clone(), new_typology: object.typology.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
