//! ↩️ Inverse for `ReplaceSection` — recovers the pre-mutation section from `base`.
use super::mutation::ReplaceSection;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSection, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.sections.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::ReplaceSection(ReplaceSection { id: payload.id.clone(), new_section: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
