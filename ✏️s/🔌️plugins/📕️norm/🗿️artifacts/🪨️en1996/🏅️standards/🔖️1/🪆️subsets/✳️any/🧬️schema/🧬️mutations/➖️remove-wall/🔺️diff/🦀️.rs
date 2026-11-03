//! ➖️ `remove-wall` diff: removes the addressed wall row.

use crate::remove_wall::RemoveWall;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &RemoveWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(index) = base.walls.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::removal(&base.walls, index), ..Default::default() })
}
