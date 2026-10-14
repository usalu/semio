//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::SetSupport;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot, diff::Patch};
pub fn inverse(payload: &SetSupport, base: &ModelSnapshot) -> Vec<ModelMutation> { base.supports.get(&payload.id).map(|record| { let patch = payload.patch().minimal(record); if patch.is_empty() { Vec::new() } else { vec![ModelMutation::SetSupport(SetSupport::from_patch(payload.id.clone(), patch.restoring(record)))] } }).unwrap_or_default() }
