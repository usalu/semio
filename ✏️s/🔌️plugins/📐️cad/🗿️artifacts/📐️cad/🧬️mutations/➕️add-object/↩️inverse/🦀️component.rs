//! ↩️ Inverse for `AddObject`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadObject, CadPaneId, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(_base: &CadSnapshot, pane: CadPaneId, object: &CadObject) -> Vec<CadMutation> {
    vec![CadMutation::RemoveObject { pane, object_id: object.id.clone() }]
}
//#endregion 🔖️Inverse
