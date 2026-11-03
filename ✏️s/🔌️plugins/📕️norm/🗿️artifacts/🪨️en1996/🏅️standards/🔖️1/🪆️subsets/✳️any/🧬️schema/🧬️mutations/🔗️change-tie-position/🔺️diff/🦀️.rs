//! 🔗️ `change-tie-position` diff: patches `position_m` of the addressed tie row.

use crate::change_tie_position::ChangeTiePosition;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996TieDelta, En1996TiePatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeTiePosition, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_position_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.ties.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tie {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.position_m == payload.new_position_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tie already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { ties: En1996TieDelta::modification(&payload.id, En1996TiePatch { position_m: Some(payload.new_position_m), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
