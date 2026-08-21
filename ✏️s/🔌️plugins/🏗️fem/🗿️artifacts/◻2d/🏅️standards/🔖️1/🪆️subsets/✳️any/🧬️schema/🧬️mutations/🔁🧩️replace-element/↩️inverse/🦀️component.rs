//! ↩️ Inverse for `ReplaceElement` — recovers the pre-mutation element from `base`.
use super::mutation::ReplaceElement;
use crate::artifacts::fem2d::element_id;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplaceElement, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.elements.iter().find(|item| element_id(item) == payload.id).map(|item| vec![Fem2dMutation::ReplaceElement(ReplaceElement { id: payload.id.clone(), new_element: Box::new(item.clone()) })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
