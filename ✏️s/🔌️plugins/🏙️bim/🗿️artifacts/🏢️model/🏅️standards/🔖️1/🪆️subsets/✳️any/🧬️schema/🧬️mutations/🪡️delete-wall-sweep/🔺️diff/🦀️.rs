//! 🔺️ Diff constructor for `DeleteWallSweep`: the sweep leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteWallSweep;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.wall_sweeps.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall sweep \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Wall sweep", Some(&payload.id))
}
