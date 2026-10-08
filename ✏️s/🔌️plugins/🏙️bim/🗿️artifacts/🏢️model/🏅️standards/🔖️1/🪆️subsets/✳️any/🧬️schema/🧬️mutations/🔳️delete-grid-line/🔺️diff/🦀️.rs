//! 🔺️ Diff constructor for `DeleteGridLine`: the grid line leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the grid line leaves.

use super::super::cascade;
use super::DeleteGridLine;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteGridLine, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.grids.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Grid line \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Grid line", Some(&payload.id))
}
