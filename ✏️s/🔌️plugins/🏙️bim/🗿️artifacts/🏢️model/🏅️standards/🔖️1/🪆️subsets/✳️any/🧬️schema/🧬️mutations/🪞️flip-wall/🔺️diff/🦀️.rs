//! 🔺️ Diff constructor for `FlipWall`: a one-field wall patch writing the reversed axis. A wall whose axis is its own reverse is a no-op.

use super::super::wall_geometry::flipped;
use super::FlipWall;
use crate::{Entry, ModelDiff, ModelSnapshot, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &FlipWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let axis = flipped(&wall.axis);
    if axis == wall.axis {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" has no direction to flip.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { axis: Some(axis), ..Default::default() })))
}
