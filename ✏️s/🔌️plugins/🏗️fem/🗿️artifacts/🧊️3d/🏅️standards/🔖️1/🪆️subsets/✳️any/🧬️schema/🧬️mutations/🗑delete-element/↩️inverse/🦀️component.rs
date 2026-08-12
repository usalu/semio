//! ↩️ Inverse for `DeleteElement` — recreates the captured element from `base`.
use super::mutation::DeleteElement;
use crate::artifacts::fem3d::element_id;
use crate::artifacts::fem3d::mutations::{create_element, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteElement, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.elements
        .iter()
        .find(|item| element_id(item) == payload.id)
        .map(|item| vec![Fem3dMutation::CreateElement(create_element::mutation::CreateElement { element: Box::new(item.clone()) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
