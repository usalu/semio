//! ↩️ Inverse for `DeleteSection` — recreates the captured section from `base`.
use super::DeleteSection;
use crate::standards::v1::subsets::any::schema::mutations::{create_section,Fem2dMutation};

use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSection, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.sections.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem2dMutation::CreateSection(create_section::CreateSection { section: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
