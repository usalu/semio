//! ↩️ Inverse for `DeleteSection` — recreates the captured section from `base`.
use super::DeleteSection;
use crate::standards::v1::subsets::any::schema::mutations::create_section;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<create_section::CreateSection>>(payload: &DeleteSection, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.sections.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(create_section::CreateSection { section: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
