//! \u{1f53a}\uFE0F Diff constructor for \u00B6CreateSchedule\u00B6: one created schedule entry. The id is free across every collection, the definition stands (a name, columns that exist in the category,
//! no repeated key, filters with their values, a scope without repeats) and every storey of the scope exists. Rows and totals are inferred, nothing derived is written.

use super::super::elements;
use super::CreateSchedule;
use crate::schedule_kit::schedule_problem;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
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
