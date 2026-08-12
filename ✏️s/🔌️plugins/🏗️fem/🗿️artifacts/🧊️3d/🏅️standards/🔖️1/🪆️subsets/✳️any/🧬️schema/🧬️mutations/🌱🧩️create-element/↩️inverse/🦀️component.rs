//! ↩️ Inverse for `CreateElement` — always a `delete-element` of the created id.
use super::mutation::CreateElement;
use crate::artifacts::fem3d::element_id;
use crate::artifacts::fem3d::mutations::{delete_element, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateElement, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: element_id(&payload.element).to_string() })]
}
//#endregion 🔖️Inverse
