//! 🔺️ Diff constructor for `SetWallEndJoin`: a one-field wall patch assigning (or clearing) the join preference of one end of the axis.
//! Restating the current preference is a no-op. Nothing derived is written: the wall layout of the wall and of the walls it touches
//! follows by inference.

use super::super::modify::WallEnd;
use super::SetWallEndJoin;
use crate::{Assigned, Entry, ModelDiff, ModelSnapshot, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallEndJoin, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let current = if payload.end == WallEnd::Start { wall.start_join } else { wall.end_join };
    if current == payload.join {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("The {:?} of wall \"{}\" already joins like this.", payload.end, payload.id), [payload.id.clone()]);
    }
    let assigned = Some(Assigned::new(payload.join));
    let patch = if payload.end == WallEnd::Start { WallPatch { start_join: assigned, ..Default::default() } } else { WallPatch { end_join: assigned, ..Default::default() } };
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(patch)))
}
