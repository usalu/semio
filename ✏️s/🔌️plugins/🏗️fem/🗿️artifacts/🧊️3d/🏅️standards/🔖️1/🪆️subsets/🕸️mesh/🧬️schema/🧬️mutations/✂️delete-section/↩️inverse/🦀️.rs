//! ↩️ Inverse for `DeleteSection` — recreates the captured section from `base`.
use super::DeleteSection;
use crate::standards::v1::subsets::any::schema::mutations::{create_section,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSection, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.sections.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem3dMutation::CreateSection(create_section::CreateSection { section: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
