//! 🔺️ Diff constructor for `SetCurtainWallTypeOf`: a one-field curtain wall patch. The curtain wall type must exist; grid, mullions and panels are inferred from it.

use super::SetCurtainWallTypeOf;
use crate::{CurtainWallPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainWallTypeOf, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.curtain_wall_types.contains_key(&payload.curtain_wall_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \"{}\" does not exist.", payload.curtain_wall_type), ["curtain_wall_type"]);
    }
    if wall.curtain_wall_type == payload.curtain_wall_type {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \"{}\" already is of type \"{}\".", payload.id, payload.curtain_wall_type), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(CurtainWallPatch { curtain_wall_type: Some(payload.curtain_wall_type.clone()), ..Default::default() })))
}
