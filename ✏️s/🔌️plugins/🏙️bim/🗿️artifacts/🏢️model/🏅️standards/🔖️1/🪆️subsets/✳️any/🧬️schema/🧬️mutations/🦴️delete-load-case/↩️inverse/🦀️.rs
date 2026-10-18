//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &DeleteLoadCase, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase>,
{ base.load_cases.get(&payload.id).map(|record| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase { id: payload.id.clone(), load_case: record.clone() })]).unwrap_or_default() }
