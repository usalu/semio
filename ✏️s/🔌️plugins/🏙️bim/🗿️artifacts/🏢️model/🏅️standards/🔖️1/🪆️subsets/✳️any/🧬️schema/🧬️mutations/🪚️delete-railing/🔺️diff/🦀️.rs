//! 🔺️ Diff constructor for `DeleteRailing`: the railing leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the railing leaves.

use super::super::cascade;
use super::DeleteRailing;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRailing, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.railings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Railing \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Railing", Some(&payload.id))
}
