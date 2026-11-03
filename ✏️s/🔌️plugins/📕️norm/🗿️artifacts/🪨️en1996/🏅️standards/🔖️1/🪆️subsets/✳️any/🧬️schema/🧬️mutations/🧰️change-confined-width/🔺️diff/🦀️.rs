//! 🧰️ `change-confined-width` diff: patches `width_m` of the addressed confined row.

use crate::change_confined_width::ChangeConfinedWidth;
use semio_s_artifact_norm_en1996_diff::{En1996ConfinedDelta, En1996ConfinedPatch, En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeConfinedWidth, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_width_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.confined.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("confined {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.width_m == payload.new_width_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The confined already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { confined: En1996ConfinedDelta::modification(&payload.id, En1996ConfinedPatch { width_m: Some(payload.new_width_m), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
