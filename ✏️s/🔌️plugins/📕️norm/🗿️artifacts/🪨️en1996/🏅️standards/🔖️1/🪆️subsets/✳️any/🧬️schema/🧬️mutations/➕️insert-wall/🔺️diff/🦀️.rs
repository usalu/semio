//! ➕️ `insert-wall` diff: inserts the wall row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_wall::InsertWall;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.walls.iter().any(|existing| existing.id == payload.wall.id) {
        let key = payload.wall.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("wall {key} already exists."), [key]);
    }
    if payload.index > base.walls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.walls.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::insertion(payload.index, payload.wall.clone()), ..Default::default() })
}
