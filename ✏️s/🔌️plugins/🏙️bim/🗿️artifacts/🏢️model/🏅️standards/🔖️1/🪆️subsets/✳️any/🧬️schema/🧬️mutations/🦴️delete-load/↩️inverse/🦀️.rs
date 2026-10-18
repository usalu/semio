//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &DeleteLoad, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_load::CreateLoad>,
{ base.loads.get(&payload.id).map(|record| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_load::CreateLoad { id: payload.id.clone(), load: record.clone() })]).unwrap_or_default() }
