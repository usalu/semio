//! ↩️ Inverse for `DeleteElement` — recreates the captured element from `base`.
use super::DeleteElement;
use crate::element_id;
use crate::standards::v1::subsets::any::schema::mutations::{create_element,Fem2dMutation};

use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteElement, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().enumerate().find(|(_, item)| element_id(item) == payload.id).map(|(at, item)| vec![Fem2dMutation::CreateElement(create_element::CreateElement { element: Box::new(item.clone()), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
