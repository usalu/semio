//! 🔺️ Diff constructor for `SetWallBaseSlab`: a one-field wall patch assigning (or clearing) the slab the base of the wall is attached to. The slab must exist in the building of the wall and the chain of attach references must
//! not loop (authored reads only); restating the current base slab is a no-op. The resolved base height follows by inference.

use super::super::wall_depth::attach_flaw;
use super::super::wall_geometry::Flaw;
use super::SetWallBaseSlab;
use crate::{Assigned, Entry, ModelDiff, ModelSnapshot, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallBaseSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if wall.base_slab == payload.slab {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already has this base.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = attach_flaw(base, Some(&payload.id), &wall.storey, &wall.top, payload.slab.as_deref()) {
        let path: Vec<&str> = flaw.path.iter().map(|segment| if segment == "base_slab" { "slab" } else { segment.as_str() }).collect();
        return Flaw::new(flaw.code, &path, flaw.message.clone()).refuse();
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { base_slab: Some(Assigned::new(payload.slab.clone())), ..Default::default() })))
}
