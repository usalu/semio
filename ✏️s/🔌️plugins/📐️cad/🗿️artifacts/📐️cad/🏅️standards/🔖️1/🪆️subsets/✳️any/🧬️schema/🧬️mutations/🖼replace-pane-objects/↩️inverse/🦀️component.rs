//! ↩️ Inverse for `ReplacePaneObjects` — restores `base`'s pane object list wholesale.
use super::mutation::ReplacePaneObjects;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplacePaneObjects, base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::ReplacePaneObjects(ReplacePaneObjects { pane: payload.pane, objects: cad_pane_objects(base, payload.pane).to_vec() })]
}
//#endregion 🔖️Inverse
