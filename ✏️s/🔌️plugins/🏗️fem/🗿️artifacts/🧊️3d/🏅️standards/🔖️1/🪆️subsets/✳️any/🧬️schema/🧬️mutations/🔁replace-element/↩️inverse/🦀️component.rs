//! ↩️ Inverse for `ReplaceElement` — recovers the pre-mutation element from `base`.
use super::mutation::ReplaceElement;
use crate::artifacts::fem3d::element_id;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceElement, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.elements
        .iter()
        .find(|item| element_id(item) == payload.id)
        .map(|item| vec![Fem3dMutation::ReplaceElement(ReplaceElement { id: payload.id.clone(), new_element: Box::new(item.clone()) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
