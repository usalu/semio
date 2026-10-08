use super::ChangeLoadCaseSituation;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeLoadCaseSituation, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid load-case index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    let load_case = &wall.load_cases[payload.load_case_index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { load_cases: En1996LoadCaseDelta::modification(&load_case.id, En1996LoadCasePatch { design_situation: Some(payload.new_design_situation.clone()), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
