//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::SetLoadCase;
use crate::{ModelMutation, ModelSnapshot, Patch};
pub fn inverse(payload: &SetLoadCase, base: &ModelSnapshot) -> Vec<ModelMutation> { base.load_cases.get(&payload.id).map(|record| { let patch = payload.patch().minimal(record); if patch.is_empty() { Vec::new() } else { vec![ModelMutation::SetLoadCase(SetLoadCase::from_patch(payload.id.clone(), patch.restoring(record)))] } }).unwrap_or_default() }
