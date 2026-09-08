//! ↩️ Inverse for `CreateLoadCase` — always a `delete-load-case` of the created id.
use super::CreateLoadCase;
use crate::mutations::{delete_load_case, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateLoadCase, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: payload.load_case.id.clone() })]
}
//#endregion 🔖️Inverse
