//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::SetLoad;
use crate::{ModelMutation, ModelSnapshot, Patch};
pub fn inverse(payload: &SetLoad, base: &ModelSnapshot) -> Vec<ModelMutation> { base.loads.get(&payload.id).map(|record| { let patch = payload.patch().minimal(record); if patch.is_empty() { Vec::new() } else { vec![ModelMutation::SetLoad(SetLoad::from_patch(payload.id.clone(), patch.restoring(record)))] } }).unwrap_or_default() }
