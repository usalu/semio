//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &CreateSupport, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport>,
{ if base.supports.contains_key(&payload.id) { return Vec::new(); } vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport { id: payload.id.clone() })] }
