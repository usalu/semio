//! ↩️ Inverse for `CreateLoadCase` — always a `delete-load-case` of the created id.
use super::mutation::CreateLoadCase;
use crate::artifacts::fem3d::mutations::{delete_load_case, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateLoadCase, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: payload.load_case.id.clone() })]
}
//#endregion 🔖️Inverse
