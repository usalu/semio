use crate::remove_concentrated::RemoveConcentrated;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_diff::{En1996ConcentratedDelta, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &RemoveConcentrated, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid concentrated remove."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    let load_case = &wall.load_cases[payload.load_case_index];
    protocol::MutationOutcome::new(En1996Diff {
        walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {
            load_cases: En1996LoadCaseDelta::modification(&load_case.id, En1996LoadCasePatch { concentrated: En1996ConcentratedDelta::removal(&load_case.concentrated, payload.index), ..Default::default() }),
            ..Default::default()
        }),
        ..Default::default()
    })
}
