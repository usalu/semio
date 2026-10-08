//! 🔺️ Diff constructor for `DeleteBeam`: the beam leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the beam leaves.

use super::super::cascade;
use super::DeleteBeam;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.beams.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Beam", Some(&payload.id))
}
