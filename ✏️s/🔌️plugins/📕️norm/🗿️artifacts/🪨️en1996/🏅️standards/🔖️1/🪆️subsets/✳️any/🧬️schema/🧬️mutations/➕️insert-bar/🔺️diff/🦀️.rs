//! ➕️ `insert-bar` diff: inserts the bar row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bar::InsertBar;
use semio_s_artifact_norm_en1996_diff::{En1996BarDelta, En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertBar, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    if r0.bars.iter().any(|existing| existing.id == payload.bar.id) {
        let key = payload.bar.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("bar {key} already exists."), [key]);
    }
    if payload.index > r0.bars.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.bars.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { bars: En1996BarDelta::insertion(payload.index, payload.bar.clone()), ..Default::default() }), ..Default::default() })
}
