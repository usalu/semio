//! ➖️ `remove-concentrated` diff: removes the addressed concentrated row.

use crate::remove_concentrated::RemoveConcentrated;
use semio_s_artifact_norm_en1996_diff::{En1996ConcentratedDelta, En1996Diff, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &RemoveConcentrated, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(r1) = r0.load_cases.iter().find(|row| row.id == payload.load_case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load case {} does not exist.", payload.load_case_id), [payload.load_case_id.clone()]);
    };
    let Some(index) = r1.concentrated.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("concentrated {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { load_cases: En1996LoadCaseDelta::modification(&payload.load_case_id, En1996LoadCasePatch { concentrated: En1996ConcentratedDelta::removal(&r1.concentrated, index), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
