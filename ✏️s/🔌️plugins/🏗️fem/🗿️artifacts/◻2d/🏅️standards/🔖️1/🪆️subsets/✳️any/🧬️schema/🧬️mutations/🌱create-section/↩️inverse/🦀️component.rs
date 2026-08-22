//! ↩️ Inverse for `CreateSection` — always a `delete-section` of the created id.
use super::mutation::CreateSection;
use crate::artifacts::fem2d::mutations::{delete_section, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSection, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: payload.section.id.clone() })]
}
//#endregion 🔖️Inverse
