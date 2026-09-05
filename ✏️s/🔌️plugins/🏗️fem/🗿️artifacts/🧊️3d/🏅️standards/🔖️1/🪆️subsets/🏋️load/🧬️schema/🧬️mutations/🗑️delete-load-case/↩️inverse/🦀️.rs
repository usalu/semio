//! ↩️ Inverse for `DeleteLoadCase` — recreates the captured load case from `base`.
use super::DeleteLoadCase;
use crate::artifacts::fem3d::mutations::{create_load_case, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteLoadCase, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.load_cases.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
