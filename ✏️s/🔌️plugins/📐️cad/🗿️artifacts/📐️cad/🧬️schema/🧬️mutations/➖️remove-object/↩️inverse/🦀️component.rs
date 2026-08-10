//! ↩️ Inverse for `RemoveObject`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadPaneId, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(base: &CadSnapshot, pane: CadPaneId, object_id: &str) -> Vec<CadMutation> {
    cad_pane_objects(base, pane)
        .iter()
        .find(|object| object.id == *object_id)
        .map(|object| vec![CadMutation::AddObject { pane, object: object.clone() }])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
