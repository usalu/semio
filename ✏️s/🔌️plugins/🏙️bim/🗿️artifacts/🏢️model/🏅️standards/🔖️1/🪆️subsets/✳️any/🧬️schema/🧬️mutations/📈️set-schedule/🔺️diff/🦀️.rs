//! \u{1f53a}\uFE0F Diff constructor for \u00B6SetSchedule\u00B6: a sparse schedule patch of exactly the fields that change. The definition that results must stand (see \u00B6schedule_problem\u00B6: columns that exist in
//! the category, no repeated key, filters with their values, a scope without repeats) and every storey of a new scope must exist; providing only equal values is a no-op. A list replaces the whole
//! list, because columns, sort keys, filters and grouping levels are one ordered definition and not addressable records.

use super::SetSchedule;
use crate::schedule_kit::schedule_problem;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(schedule) = base.schedules.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Schedule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some((path, message)) = schedule_problem(&payload.patch().write(schedule)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [path]);
    }
    if let Some(storey) = payload.storeys.iter().flatten().find(|storey| !base.storeys.contains_key(*storey)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{storey}\" does not exist."), ["storeys"]);
    }
    let patch = payload.patch().minimal(schedule);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Schedule \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::schedules(payload.id.clone(), Entry::Patched(patch)))
}
