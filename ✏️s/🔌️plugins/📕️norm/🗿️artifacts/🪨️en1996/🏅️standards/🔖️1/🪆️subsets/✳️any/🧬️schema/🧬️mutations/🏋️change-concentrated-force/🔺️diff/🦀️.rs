use super::ChangeConcentratedForce;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996ConcentratedDelta, En1996ConcentratedPatch, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeConcentratedForce, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid concentrated index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    let load_case = &wall.load_cases[payload.load_case_index];
    let concentrated = &load_case.concentrated[payload.index];
    protocol::MutationOutcome::new(En1996Diff {
        walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {
            load_cases: En1996LoadCaseDelta::modification(&load_case.id, En1996LoadCasePatch { concentrated: En1996ConcentratedDelta::modification(&concentrated.id, En1996ConcentratedPatch { force_n: Some(payload.new_force_n), ..Default::default() }), ..Default::default() }),
            ..Default::default()
        }),
        ..Default::default()
    })
}
