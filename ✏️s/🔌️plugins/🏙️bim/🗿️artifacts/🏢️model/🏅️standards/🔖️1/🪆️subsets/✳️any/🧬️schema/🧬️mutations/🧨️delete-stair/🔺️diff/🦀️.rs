//! 🔺️ Diff constructor for `DeleteStair`: the stair leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the stair leaves.

use super::super::cascade;
use super::DeleteStair;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteStair, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.stairs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Stair \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Stair", Some(&payload.id))
}
