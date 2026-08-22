//! ↩️ Inverse for `DeleteElement` — recreates the captured element from `base`.
use super::mutation::DeleteElement;
use crate::artifacts::fem2d::element_id;
use crate::artifacts::fem2d::mutations::{create_element, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteElement, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.elements.iter().find(|item| element_id(item) == payload.id).map(|item| vec![Fem2dMutation::CreateElement(create_element::mutation::CreateElement { element: Box::new(item.clone()) })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
