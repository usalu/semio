//! ➖️ `remove-load-case` diff: removes the addressed load case row.

use crate::remove_load_case::RemoveLoadCase;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996LoadCaseDelta, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &RemoveLoadCase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.wall_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.wall_id), [payload.wall_id.clone()]);
    };
    let Some(index) = r0.load_cases.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.wall_id, En1996WallPatch { load_cases: En1996LoadCaseDelta::removal(&r0.load_cases, index), ..Default::default() }), ..Default::default() })
}
