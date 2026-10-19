//! 🔺️ Diff constructor for \u00B6CreateSchedule\u00B6: one created schedule entry. The id is free across every collection, the definition stands (a name, columns that exist in the category,
//! no repeated key, filters with their values, a scope without repeats) and every storey of the scope exists. Rows and totals are inferred, nothing derived is written.

use crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::schedule_kit::schedule_problem;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some((path, message)) = schedule_problem(&payload.schedule) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["schedule", path]);
    }
    if let Some(storey) = payload.schedule.storeys.iter().find(|storey| !base.storeys.contains_key(*storey)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{storey}\" does not exist."), ["schedule", "storeys"]);
    }
    MutationOutcome::new(ModelDiff::schedules(payload.id.clone(), Entry::Created(payload.schedule.clone())))
}
