//! 🔺️ Diff constructor for `DeleteOpening`: the opening leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the opening leaves.

use super::super::cascade;
use super::DeleteOpening;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteOpening, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.openings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Opening \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Opening", Some(&payload.id))
}
