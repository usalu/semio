//! 🔺️ Diff constructor for `DeleteWallSweep`: the sweep leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_wall_sweep::DeleteWallSweep;
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.wall_sweeps.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall sweep \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Wall sweep", Some(&payload.id))
}
