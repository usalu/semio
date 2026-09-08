//! ↩️ Inverse for `CreateElement` — always a `delete-element` of the created id.
use super::CreateElement;
use crate::element_id;
use crate::standards::v1::subsets::any::schema::mutations::{delete_element, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateElement, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteElement(delete_element::DeleteElement { id: element_id(&payload.element).to_string() })]
}
//#endregion 🔖️Inverse
