//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::DeleteLoadCase;
use crate::{ModelMutation, ModelSnapshot, Patch};
pub fn inverse(payload: &DeleteLoadCase, base: &ModelSnapshot) -> Vec<ModelMutation> { base.load_cases.get(&payload.id).map(|record| vec![ModelMutation::CreateLoadCase(super::super::create_load_case::CreateLoadCase { id: payload.id.clone(), load_case: record.clone() })]).unwrap_or_default() }
