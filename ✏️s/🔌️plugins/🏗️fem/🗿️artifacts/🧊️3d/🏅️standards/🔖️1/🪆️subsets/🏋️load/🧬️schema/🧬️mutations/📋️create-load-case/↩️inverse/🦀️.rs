//! ↩️ Inverse for `CreateLoadCase` — always a `delete-load-case` of the created id.
use super::CreateLoadCase;
use crate::standards::v1::subsets::any::schema::mutations::{delete_load_case, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateLoadCase, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: payload.load_case.id.clone() })]
}
//#endregion 🔖️Inverse
