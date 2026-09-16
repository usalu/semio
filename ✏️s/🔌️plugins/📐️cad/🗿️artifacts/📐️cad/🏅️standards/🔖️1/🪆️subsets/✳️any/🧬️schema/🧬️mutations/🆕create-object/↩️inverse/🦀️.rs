//! ↩️ Inverse for `CreateObject` — a `delete-object` of the created id in the same pane.
use super::CreateObject;
use crate::mutations::{delete_object, CadMutation};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateObject, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DeleteObject(delete_object::DeleteObject { pane: payload.pane, object_id: payload.object.id.clone() })]
}
//#endregion 🔖️Inverse
