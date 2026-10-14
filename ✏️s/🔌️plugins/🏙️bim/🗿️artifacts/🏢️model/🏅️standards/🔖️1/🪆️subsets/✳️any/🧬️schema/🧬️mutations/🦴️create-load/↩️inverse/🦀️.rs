//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::CreateLoad;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
pub fn inverse(payload: &CreateLoad, base: &ModelSnapshot) -> Vec<ModelMutation> { if base.loads.contains_key(&payload.id) { return Vec::new(); } vec![ModelMutation::DeleteLoad(super::super::delete_load::DeleteLoad { id: payload.id.clone() })] }
