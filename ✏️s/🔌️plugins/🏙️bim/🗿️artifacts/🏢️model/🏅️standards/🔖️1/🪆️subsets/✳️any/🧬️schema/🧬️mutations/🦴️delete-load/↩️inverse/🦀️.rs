//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::DeleteLoad;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
pub fn inverse(payload: &DeleteLoad, base: &ModelSnapshot) -> Vec<ModelMutation> { base.loads.get(&payload.id).map(|record| vec![ModelMutation::CreateLoad(super::super::create_load::CreateLoad { id: payload.id.clone(), load: record.clone() })]).unwrap_or_default() }
