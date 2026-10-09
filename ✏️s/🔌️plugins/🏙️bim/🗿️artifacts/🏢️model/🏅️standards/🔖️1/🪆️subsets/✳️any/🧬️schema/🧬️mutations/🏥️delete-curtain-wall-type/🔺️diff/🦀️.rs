//! 🔺️ Diff constructor for `DeleteCurtainWallType`: one deleted curtain wall type entry; refused while a curtain wall still uses the type.

use super::DeleteCurtainWallType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.curtain_wall_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.curtain_walls.values().any(|row| row.curtain_wall_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Curtain wall type \"{}\" is still used by curtain walls.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Deleted))
}
