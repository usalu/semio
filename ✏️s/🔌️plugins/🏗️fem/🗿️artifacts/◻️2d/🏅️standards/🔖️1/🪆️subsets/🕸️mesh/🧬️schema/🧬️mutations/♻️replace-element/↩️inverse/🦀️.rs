//! ↩️ Inverse for `ReplaceElement` — recovers the pre-mutation element from `base`.
use super::ReplaceElement;
use crate::element_id;
use crate::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceElement, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.elements.iter().find(|item| element_id(item) == payload.id).map(|item| vec![Fem2dMutation::ReplaceElement(ReplaceElement { id: payload.id.clone(), new_element: Box::new(item.clone()) })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
