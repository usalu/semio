//! ↩️ Inverse for `DeleteElement` — recreates the captured element from `base`.
use super::DeleteElement;
use crate::element_id;
use crate::mutations::{create_element, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteElement, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.elements.iter().find(|item| element_id(item) == payload.id).map(|item| vec![Fem2dMutation::CreateElement(create_element::CreateElement { element: Box::new(item.clone()) })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
