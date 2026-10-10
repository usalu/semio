//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::CreateLoadCase;
use crate::{ModelMutation, ModelSnapshot, Patch};
pub fn inverse(payload: &CreateLoadCase, base: &ModelSnapshot) -> Vec<ModelMutation> { if base.load_cases.contains_key(&payload.id) { return Vec::new(); } vec![ModelMutation::DeleteLoadCase(super::super::delete_load_case::DeleteLoadCase { id: payload.id.clone() })] }
