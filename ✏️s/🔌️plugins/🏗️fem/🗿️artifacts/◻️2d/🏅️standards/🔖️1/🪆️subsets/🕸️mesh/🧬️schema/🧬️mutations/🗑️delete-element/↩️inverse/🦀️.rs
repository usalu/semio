//! ↩️ Inverse for `DeleteElement` — recreates the captured element from `base`.
use super::DeleteElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use crate::standards::v1::subsets::any::schema::mutations::create_element;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<create_element::CreateElement>>(payload: &DeleteElement, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().enumerate().find(|(_, item)| element_id(item) == payload.id).map(|(at, item)| vec![Op::from(create_element::CreateElement { element: Box::new(item.clone()), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
