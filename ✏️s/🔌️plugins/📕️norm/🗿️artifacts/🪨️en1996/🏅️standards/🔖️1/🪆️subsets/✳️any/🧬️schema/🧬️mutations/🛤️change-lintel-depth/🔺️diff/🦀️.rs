//! 🛤️ `change-lintel-depth` diff: patches `depth_m` of the addressed lintel row.

use crate::change_lintel_depth::ChangeLintelDepth;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996LintelDelta, En1996LintelPatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeLintelDepth, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_depth_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.lintels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lintel {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.depth_m == payload.new_depth_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The lintel already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { lintels: En1996LintelDelta::modification(&payload.id, En1996LintelPatch { depth_m: Some(payload.new_depth_m), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
