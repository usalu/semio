//! 🧰️ `change-confined-steel` diff: patches `as_m2` of the addressed confined row.

use crate::change_confined_steel::ChangeConfinedSteel;
use semio_s_artifact_norm_en1996_diff::{En1996ConfinedDelta, En1996ConfinedPatch, En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeConfinedSteel, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_as_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.confined.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("confined {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.as_m2 == payload.new_as_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The confined already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { confined: En1996ConfinedDelta::modification(&payload.id, En1996ConfinedPatch { as_m2: Some(payload.new_as_m2), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
