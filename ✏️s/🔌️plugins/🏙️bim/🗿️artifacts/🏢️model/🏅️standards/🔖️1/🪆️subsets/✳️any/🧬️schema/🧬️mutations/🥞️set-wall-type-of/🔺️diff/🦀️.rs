//! 🔺️ Diff constructor for `SetWallTypeOf`: a one-field wall patch. The wall type must exist; thickness and layers are inferred from it.

use super::SetWallTypeOf;
use crate::{Entry, ModelDiff, ModelSnapshot, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallTypeOf, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.wall_types.contains_key(&payload.wall_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall type \"{}\" does not exist.", payload.wall_type), ["wall_type"]);
    }
    if wall.wall_type == payload.wall_type {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already is of type \"{}\".", payload.id, payload.wall_type), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { wall_type: Some(payload.wall_type.clone()), ..Default::default() })))
}
