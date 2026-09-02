//! ↩️ Inverse for `CreateSection` — always a `delete-section` of the created id.
use super::CreateSection;
use crate::artifacts::fem3d::mutations::{delete_section, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSection, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteSection(delete_section::DeleteSection { id: payload.section.id.clone() })]
}
//#endregion 🔖️Inverse
