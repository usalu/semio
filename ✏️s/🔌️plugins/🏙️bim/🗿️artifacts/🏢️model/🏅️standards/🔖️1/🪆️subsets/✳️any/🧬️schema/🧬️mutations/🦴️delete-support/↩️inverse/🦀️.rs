//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &DeleteSupport, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport>,
{ base.supports.get(&payload.id).map(|record| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport { id: payload.id.clone(), support: record.clone() })]).unwrap_or_default() }
