//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::set_support::SetSupport;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};
pub fn inverse<Op>(payload: &SetSupport, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_support::SetSupport>,
{ base.supports.get(&payload.id).map(|record| { let patch = payload.patch().minimal(record); if patch.is_empty() { Vec::new() } else { vec![Op::from(SetSupport::from_patch(payload.id.clone(), patch.restoring(record)))] } }).unwrap_or_default() }
