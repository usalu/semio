//! ↩️ Inverse for `DeleteSection` — recreates the captured section from `base`.
use super::mutation::DeleteSection;
use crate::artifacts::fem2d::mutations::{create_section, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSection, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.sections.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateSection(create_section::mutation::CreateSection { section: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
