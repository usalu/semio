//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::DeleteLoad;
use crate::{ModelMutation, ModelSnapshot, Patch};
pub fn inverse(payload: &DeleteLoad, base: &ModelSnapshot) -> Vec<ModelMutation> { base.loads.get(&payload.id).map(|record| vec![ModelMutation::CreateLoad(super::super::create_load::CreateLoad { id: payload.id.clone(), load: record.clone() })]).unwrap_or_default() }
