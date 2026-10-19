//! ↩️ Inverse for `DeleteElement` — recreates the captured element from `base`.
use super::DeleteElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use crate::standards::v1::subsets::any::schema::mutations::create_element;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::create_element::CreateElement>>(payload: &DeleteElement, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().enumerate().find(|(_, item)| element_id(item) == payload.id).map(|(at, item)| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_element::CreateElement { element: Box::new(item.clone()), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
