//! ➖️ `remove-retaining-wall` diff: removes the addressed retaining wall row.

use crate::remove_retaining_wall::RemoveRetainingWall;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998RetainingWallDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.retaining_walls.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::removal(&base.retaining_walls, index), ..Default::default() })
}
