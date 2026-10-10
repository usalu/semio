//! 🔺️ Diff constructor for `SetSpaceConditions`: the first use creates the record of the space from the provided fields, later uses a sparse patch of exactly the provided fields that differ. The space must exist and the
//! resulting conditions must be writable (see `conditions_problem`); providing only equal values is a no-op.

use super::SetSpaceConditions;
use crate::{conditions_problem, Entry, ModelDiff, ModelSnapshot, Patch, SpaceConditions};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSpaceConditions, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let patch = payload.patch();
    match base.space_conditions.get(&payload.id) {
        None => {
            let record = patch.write(&SpaceConditions::empty());
            if let Some((field, message)) = conditions_problem(&record) {
                return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
            }
            MutationOutcome::new(ModelDiff::space_conditions(payload.id.clone(), Entry::Created(record)))
        }
        Some(record) => {
            let change = patch.minimal(record);
            if change.is_empty() {
                return MutationOutcome::refuse(OutcomeCode::NoOp, format!("The conditions of space \"{}\" already have these values.", payload.id), [payload.id.clone()]);
            }
            if let Some((field, message)) = conditions_problem(&change.write(record)) {
                return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
            }
            MutationOutcome::new(ModelDiff::space_conditions(payload.id.clone(), Entry::Patched(change)))
        }
    }
}
