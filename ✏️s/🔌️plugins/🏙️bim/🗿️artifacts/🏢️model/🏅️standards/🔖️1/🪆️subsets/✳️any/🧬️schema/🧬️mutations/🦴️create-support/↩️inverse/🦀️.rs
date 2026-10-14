//! ↩️ Concrete semantic inverse restoring only authored parameters.
use super::CreateSupport;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
pub fn inverse(payload: &CreateSupport, base: &ModelSnapshot) -> Vec<ModelMutation> { if base.supports.contains_key(&payload.id) { return Vec::new(); } vec![ModelMutation::DeleteSupport(super::super::delete_support::DeleteSupport { id: payload.id.clone() })] }
