//! ↩️ Inverse for `CreateElement` — always a `delete-element` of the created id.
use super::CreateElement;
use crate::element_id;
use crate::standards::v1::subsets::any::schema::mutations::{delete_element, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateElement, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteElement(delete_element::DeleteElement { id: element_id(&payload.element).to_string() })]
}
//#endregion 🔖️Inverse
