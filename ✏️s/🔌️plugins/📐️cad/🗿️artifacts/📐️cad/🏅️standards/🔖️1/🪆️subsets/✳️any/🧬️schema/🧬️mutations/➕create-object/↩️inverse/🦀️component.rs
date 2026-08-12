//! ↩️ Inverse for `CreateObject` — always a `delete-object` of the created id.
use super::mutation::CreateObject;
use crate::artifacts::cad::mutations::delete_object;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateObject, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DeleteObject(delete_object::mutation::DeleteObject { pane: payload.pane, object_id: payload.object.id.clone() })]
}
//#endregion 🔖️Inverse
