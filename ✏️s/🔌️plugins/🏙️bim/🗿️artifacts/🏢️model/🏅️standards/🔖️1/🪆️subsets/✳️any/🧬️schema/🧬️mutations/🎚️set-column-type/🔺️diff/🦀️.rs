//! 🔺️ Diff constructor for `SetColumnType`: a sparse column type patch of exactly the provided fields. A provided material
//! must exist and every provided value must keep the profile a valid cross-section; providing nothing new is a no-op.

use super::SetColumnType;
use crate::{profile_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetColumnType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.column_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(material) = payload.material.as_ref().filter(|material| !base.materials.contains_key(*material)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{material}\" does not exist."), ["material"]);
    }
    if let Some(problem) = payload.profile.as_ref().and_then(profile_problem) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, problem, ["profile"]);
    }
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Column type \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::column_types(payload.id.clone(), Entry::Patched(patch)))
}
