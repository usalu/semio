//! ✏️ `change-retaining-wall-r` diff: patches `r` of the addressed retaining wall row.

use crate::change_retaining_wall_r::ChangeRetainingWallR;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998RetainingWallDelta, En1998RetainingWallPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeRetainingWallR, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_r.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.r == payload.new_r {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::modification(&payload.id, En1998RetainingWallPatch { r: Some(payload.new_r), ..Default::default() }), ..Default::default() })
}
