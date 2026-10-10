//! 🔺️ Diff constructor for `DeleteComponent`: the component leaves in one sparse diff together with its parameter overrides and its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the component leaves.

use super::super::cascade;
use super::DeleteComponent;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteComponent, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.components.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Component", Some(&payload.id))
}
