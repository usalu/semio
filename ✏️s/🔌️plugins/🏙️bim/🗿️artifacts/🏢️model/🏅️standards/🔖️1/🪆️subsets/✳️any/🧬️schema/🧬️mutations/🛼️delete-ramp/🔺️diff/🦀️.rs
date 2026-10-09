//! 🔺️ Diff constructor for `DeleteRamp`: the ramp leaves in one sparse diff together with the railings hosted by it and its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the ramp leaves.

use super::super::cascade;
use super::DeleteRamp;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRamp, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.ramps.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ramp \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Ramp", Some(&payload.id))
}
