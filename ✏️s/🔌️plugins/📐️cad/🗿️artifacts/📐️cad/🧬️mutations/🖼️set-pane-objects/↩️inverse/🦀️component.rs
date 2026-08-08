//! ↩️ Inverse for `SetPaneObjects`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadObject, CadPaneId, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(base: &CadSnapshot, pane: CadPaneId, _objects: &[CadObject]) -> Vec<CadMutation> {
    vec![CadMutation::SetPaneObjects { pane, objects: cad_pane_objects(base, pane).to_vec() }]
}
//#endregion 🔖️Inverse
