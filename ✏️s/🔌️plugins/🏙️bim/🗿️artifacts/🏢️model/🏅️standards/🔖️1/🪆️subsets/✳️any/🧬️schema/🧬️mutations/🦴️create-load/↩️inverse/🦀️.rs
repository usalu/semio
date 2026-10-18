//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::create_load::CreateLoad;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &CreateLoad, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad>,
{ if base.loads.contains_key(&payload.id) { return Vec::new(); } vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad { id: payload.id.clone() })] }
