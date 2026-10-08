//! 🔺️ Diff constructor for `RenameElement`: a one-field patch of the name (a grid line's label) in whichever collection holds the id.

use super::super::elements;
use super::RenameElement;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RenameElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some((current, diff)) = elements::rename(base, &payload.id, &payload.name) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if current == payload.name {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Element \"{}\" is already named \"{}\".", payload.id, payload.name), [payload.id.clone()]);
    }
    MutationOutcome::new(diff)
}
