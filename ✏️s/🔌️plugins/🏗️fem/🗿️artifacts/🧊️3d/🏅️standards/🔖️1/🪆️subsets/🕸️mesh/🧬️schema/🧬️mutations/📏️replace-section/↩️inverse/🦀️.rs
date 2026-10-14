//! ↩️ Inverse for `ReplaceSection` — recovers the pre-mutation section from `base`.
use super::ReplaceSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<replace_section::ReplaceSection>>(payload: &ReplaceSection, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.sections.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceSection { id: payload.id.clone(), new_section: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
