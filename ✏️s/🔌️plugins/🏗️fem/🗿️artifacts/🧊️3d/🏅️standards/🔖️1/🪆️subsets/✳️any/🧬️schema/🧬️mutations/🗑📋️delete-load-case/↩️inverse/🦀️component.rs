//! ↩️ Inverse for `DeleteLoadCase` — recreates the captured load case from `base`.
use super::mutation::DeleteLoadCase;
use crate::artifacts::fem3d::mutations::{create_load_case, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteLoadCase, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.load_cases.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
