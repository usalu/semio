//! 🔺️ Diff constructor for `DeleteMepElement`: the MEP element leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteMepElement;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.mep_elements.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("MEP element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "MEP element", Some(&payload.id))
}
