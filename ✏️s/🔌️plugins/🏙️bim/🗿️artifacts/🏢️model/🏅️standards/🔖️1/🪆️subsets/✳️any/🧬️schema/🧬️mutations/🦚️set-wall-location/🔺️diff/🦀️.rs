//! 🔺️ Diff constructor for `SetWallLocation`: a one-field wall patch. The offset curves of the wall faces are inferred from it.

use super::SetWallLocation;
use crate::{Entry, ModelDiff, ModelSnapshot, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallLocation, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if wall.location == payload.location {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already is located on this line.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { location: Some(payload.location), ..Default::default() })))
}
