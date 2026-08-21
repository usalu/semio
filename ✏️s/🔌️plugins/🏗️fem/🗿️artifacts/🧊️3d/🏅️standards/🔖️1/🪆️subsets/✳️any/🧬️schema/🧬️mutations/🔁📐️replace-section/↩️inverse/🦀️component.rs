//! ↩️ Inverse for `ReplaceSection` — recovers the pre-mutation section from `base`.
use super::mutation::ReplaceSection;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplaceSection, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.sections.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::ReplaceSection(ReplaceSection { id: payload.id.clone(), new_section: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
