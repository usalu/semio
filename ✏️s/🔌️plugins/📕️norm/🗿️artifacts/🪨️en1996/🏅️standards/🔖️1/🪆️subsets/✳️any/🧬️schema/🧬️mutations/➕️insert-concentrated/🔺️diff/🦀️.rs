//! ➕️ `insert-concentrated` diff: inserts the concentrated row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_concentrated::InsertConcentrated;
use semio_s_artifact_norm_en1996_diff::{En1996ConcentratedDelta, En1996Diff, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertConcentrated, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.load_cases.iter().find(|row| row.id == payload.load_case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load case {} does not exist.", payload.load_case_id), [payload.load_case_id.clone()]);
    };
    if r1.concentrated.iter().any(|existing| existing.id == payload.load.id) {
        let key = payload.load.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("concentrated {key} already exists."), [key]);
    }
    if payload.index > r1.concentrated.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r1.concentrated.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { load_cases: En1996LoadCaseDelta::modification(&payload.load_case_id, En1996LoadCasePatch { concentrated: En1996ConcentratedDelta::insertion(payload.index, payload.load.clone()), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
