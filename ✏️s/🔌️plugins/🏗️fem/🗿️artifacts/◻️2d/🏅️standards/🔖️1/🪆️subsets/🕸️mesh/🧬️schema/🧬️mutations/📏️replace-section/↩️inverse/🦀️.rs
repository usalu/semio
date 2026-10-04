//! ↩️ Inverse for `ReplaceSection` — recovers the pre-mutation section from `base`.
use super::ReplaceSection;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSection, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.sections.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::ReplaceSection(ReplaceSection { id: payload.id.clone(), new_section: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
