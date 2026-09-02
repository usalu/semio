//! ↩️ Inverse for `DeleteLoadCase` — recreates the captured load case from `base`.
use super::DeleteLoadCase;
use crate::artifacts::fem2d::mutations::{create_load_case, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteLoadCase, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.load_cases.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
