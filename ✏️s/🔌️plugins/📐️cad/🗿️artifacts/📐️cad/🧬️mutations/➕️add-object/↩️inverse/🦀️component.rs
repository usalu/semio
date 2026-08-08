//! ↩️ Inverse for `AddObject`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadObject, CadPaneId, CadProjection};

//#region 🔖️Inverse
pub fn inverse(_base: &CadProjection, pane: CadPaneId, object: &CadObject) -> Vec<CadMutation> {
    vec![CadMutation::RemoveObject { pane, object_id: object.id.clone() }]
}
//#endregion 🔖️Inverse
