//! ↩️ Inverse for `CreateElement` — always a `delete-element` of the created id.
use super::CreateElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use crate::standards::v1::subsets::any::schema::mutations::delete_element;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement>>(payload: &CreateElement, _base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement { id: element_id(&payload.element).to_string() })]

    })())
}
//#endregion 🔖️Inverse
