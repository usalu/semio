use super::InsertConcentrated;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996ConcentratedDelta, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &InsertConcentrated, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index > base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid concentrated insert."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    let load_case = &wall.load_cases[payload.load_case_index];
    if load_case.concentrated.iter().any(|existing| existing.id == payload.load.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.load.id), [payload.load.id.clone()]);
    }
    protocol::MutationOutcome::new(En1996Diff {
        walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {
            load_cases: En1996LoadCaseDelta::modification(&load_case.id, En1996LoadCasePatch { concentrated: En1996ConcentratedDelta::insertion(&load_case.concentrated, payload.index, payload.load.clone()), ..Default::default() }),
            ..Default::default()
        }),
        ..Default::default()
    })
}
