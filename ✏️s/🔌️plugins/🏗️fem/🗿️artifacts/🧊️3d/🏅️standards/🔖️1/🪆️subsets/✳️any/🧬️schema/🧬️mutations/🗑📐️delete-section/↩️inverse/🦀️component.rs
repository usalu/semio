//! ↩️ Inverse for `DeleteSection` — recreates the captured section from `base`.
use super::mutation::DeleteSection;
use crate::artifacts::fem3d::mutations::{create_section, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSection, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.sections.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateSection(create_section::mutation::CreateSection { section: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
