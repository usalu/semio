//! ↩️ Concrete semantic inverse restoring only authored parameters.
use crate::standards::v1::subsets::any::schema::mutations::set_load_case::SetLoadCase;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;
pub fn inverse<Op>(payload: &SetLoadCase, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_load_case::SetLoadCase>,
{ base.load_cases.get(&payload.id).map(|record| { let patch = payload.patch().minimal(record); if patch.is_empty() { Vec::new() } else { vec![Op::from(SetLoadCase::from_patch(payload.id.clone(), patch.restoring(record)))] } }).unwrap_or_default() }
