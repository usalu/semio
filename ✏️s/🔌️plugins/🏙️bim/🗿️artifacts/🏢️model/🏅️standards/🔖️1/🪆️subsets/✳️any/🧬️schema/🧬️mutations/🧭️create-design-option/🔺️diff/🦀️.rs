//! 🔺️ Sparse declarative create-design-option diff.
use crate::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &CreateDesignOption, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if payload.id.trim().is_empty() { return MutationOutcome::refuse(OutcomeCode::Invariant, "Id is empty.", [payload.id.clone()]); }
    if semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id).is_some() { return MutationOutcome::refuse(OutcomeCode::DuplicateId, "Id already exists.", [payload.id.clone()]); }
    if let Some(problem) = semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::authored::option_rules::option_problem(base, &payload.id, &payload.design_option) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::design_options(payload.id.clone(), Entry::Created(payload.design_option.clone())))
}
