//! ➕️ `insert-confined` diff: inserts the confined row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_confined::InsertConfined;
use semio_s_artifact_norm_en1996_diff::{En1996ConfinedDelta, En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertConfined, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    if r0.confined.iter().any(|existing| existing.id == payload.confined.id) {
        let key = payload.confined.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("confined {key} already exists."), [key]);
    }
    if payload.index > r0.confined.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.confined.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { confined: En1996ConfinedDelta::insertion(payload.index, payload.confined.clone()), ..Default::default() }), ..Default::default() })
}
