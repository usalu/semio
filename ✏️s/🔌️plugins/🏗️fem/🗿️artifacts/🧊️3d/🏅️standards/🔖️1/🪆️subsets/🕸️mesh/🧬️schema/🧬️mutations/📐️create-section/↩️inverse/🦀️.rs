//! ↩️ Inverse for `CreateSection` — always a `delete-section` of the created id.
use super::CreateSection;
use crate::standards::v1::subsets::any::schema::mutations::delete_section;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<delete_section::DeleteSection>>(payload: &CreateSection, _base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_section::DeleteSection { id: payload.section.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
