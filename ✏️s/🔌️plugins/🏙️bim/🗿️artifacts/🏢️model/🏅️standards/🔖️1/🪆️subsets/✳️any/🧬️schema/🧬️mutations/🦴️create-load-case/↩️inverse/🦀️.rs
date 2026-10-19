//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &CreateLoadCase, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase>,
{ if base.load_cases.contains_key(&payload.id) { return Vec::new(); } vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase { id: payload.id.clone() })] }
