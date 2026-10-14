//! ↩️ Inverse for `ReplaceElement` — recovers the pre-mutation element from `base`.
use super::ReplaceElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<replace_element::ReplaceElement>>(payload: &ReplaceElement, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|item| element_id(item) == payload.id).map(|item| vec![Op::from(ReplaceElement { id: payload.id.clone(), new_element: Box::new(item.clone()) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
