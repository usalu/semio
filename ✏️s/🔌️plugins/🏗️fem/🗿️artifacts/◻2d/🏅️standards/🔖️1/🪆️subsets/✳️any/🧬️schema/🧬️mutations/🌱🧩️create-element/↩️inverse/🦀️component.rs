//! ↩️ Inverse for `CreateElement` — always a `delete-element` of the created id.
use super::mutation::CreateElement;
use crate::artifacts::fem2d::element_id;
use crate::artifacts::fem2d::mutations::{delete_element, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateElement, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: element_id(&payload.element).to_string() })]
}
//#endregion 🔖️Inverse
