//! 🔺️ Diff constructor for `SetWallBaseOffset`: a one-field wall patch. The resolved base, top and height are inferred, never written.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_base_offset::SetWallBaseOffset;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff, patches::WallPatch}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallBaseOffset, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.base_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A base offset must be a finite length.", ["base_offset"]);
    }
    if wall.base_offset == payload.base_offset {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already sits {} m above its floor.", payload.id, payload.base_offset), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { base_offset: Some(payload.base_offset), ..Default::default() })))
}
