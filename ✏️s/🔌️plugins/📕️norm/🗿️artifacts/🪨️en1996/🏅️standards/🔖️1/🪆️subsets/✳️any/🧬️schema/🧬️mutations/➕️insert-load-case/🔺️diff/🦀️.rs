//! ➕️ `insert-load-case` diff: inserts the load case row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_load_case::InsertLoadCase;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996LoadCaseDelta, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertLoadCase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    if r0.load_cases.iter().any(|existing| existing.id == payload.load_case.id) {
        let key = payload.load_case.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("load case {key} already exists."), [key]);
    }
    if payload.index > r0.load_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.load_cases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { load_cases: En1996LoadCaseDelta::insertion(payload.index, payload.load_case.clone()), ..Default::default() }), ..Default::default() })
}
