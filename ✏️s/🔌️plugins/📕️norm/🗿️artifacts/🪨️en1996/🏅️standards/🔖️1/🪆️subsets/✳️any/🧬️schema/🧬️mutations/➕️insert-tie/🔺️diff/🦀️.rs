//! ➕️ `insert-tie` diff: inserts the tie row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_tie::InsertTie;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996TieDelta, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertTie, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    if r0.ties.iter().any(|existing| existing.id == payload.tie.id) {
        let key = payload.tie.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("tie {key} already exists."), [key]);
    }
    if payload.index > r0.ties.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.ties.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { ties: En1996TieDelta::insertion(payload.index, payload.tie.clone()), ..Default::default() }), ..Default::default() })
}
