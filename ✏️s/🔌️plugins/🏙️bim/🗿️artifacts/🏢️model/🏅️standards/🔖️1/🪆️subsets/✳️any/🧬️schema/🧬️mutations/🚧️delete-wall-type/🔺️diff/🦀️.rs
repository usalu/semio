//! 🔺️ Diff constructor for `DeleteWallType`: one deleted wall type entry; refused while a wall still uses the type.

use super::DeleteWallType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.wall_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.walls.values().any(|row| row.wall_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Wall type \"{}\" is still used by walls.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::wall_types(payload.id.clone(), Entry::Deleted))
}
