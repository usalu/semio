//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::CreateLoad;
use crate::{ModelMutation, ModelSnapshot, Patch};
pub fn inverse(payload: &CreateLoad, base: &ModelSnapshot) -> Vec<ModelMutation> { if base.loads.contains_key(&payload.id) { return Vec::new(); } vec![ModelMutation::DeleteLoad(super::super::delete_load::DeleteLoad { id: payload.id.clone() })] }
