//! 🔺️ Diff constructor for `SetClassificationSystem`: a sparse system patch of exactly the provided fields that differ from the base. The name stays non-blank and the entry table must stay sound (unique codes, parents
//! that are rows, no cycle); element classifications whose code the new table lacks stay and are reported by a diagnostic. Providing only equal values is a no-op.

use super::SetClassificationSystem;
use crate::{entries_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetClassificationSystem, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.classification_systems.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Classification system \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.name.as_deref().is_some_and(|name| name.trim().is_empty()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A classification system needs a name.", ["name"]);
    }
    if let Some(message) = payload.entries.as_deref().and_then(entries_problem) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["entries"]);
    }
    let minimal = payload.patch().minimal(record);
    if minimal.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Classification system \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::classification_systems(payload.id.clone(), Entry::Patched(minimal)))
}
