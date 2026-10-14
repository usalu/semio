//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::DeleteSupport;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
pub fn inverse(payload: &DeleteSupport, base: &ModelSnapshot) -> Vec<ModelMutation> { base.supports.get(&payload.id).map(|record| vec![ModelMutation::CreateSupport(super::super::create_support::CreateSupport { id: payload.id.clone(), support: record.clone() })]).unwrap_or_default() }
