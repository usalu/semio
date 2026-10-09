//! 🔺️ Diff constructor for `CreateClassificationSystem`: one created system entry. The id must be free in every collection (and not `entry`, the tag of a diff entry, which would collide with the system id used as a key of
//! an element's classifications), the system needs a name and an entry table with unique codes whose parents are rows of the table and form a forest.

use super::super::elements;
use super::CreateClassificationSystem;
use crate::{entries_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateClassificationSystem, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if payload.id == "entry" {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "The id \"entry\" is reserved: it tags the entries of a diff.", ["id"]);
    }
    let system = &payload.system;
    if system.name.trim().is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A classification system needs a name.", ["system", "name"]);
    }
    if let Some(message) = entries_problem(&system.entries) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["system", "entries"]);
    }
    MutationOutcome::new(ModelDiff::classification_systems(payload.id.clone(), Entry::Created(system.clone())))
}
