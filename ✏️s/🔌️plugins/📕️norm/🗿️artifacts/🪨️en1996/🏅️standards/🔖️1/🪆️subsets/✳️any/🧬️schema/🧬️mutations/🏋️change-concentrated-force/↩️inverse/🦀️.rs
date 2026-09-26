use super::ChangeConcentratedForce;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeConcentratedForce, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() { Vec::new() } else { vec![En1996Mutation::ChangeConcentratedForce(ChangeConcentratedForce { wall_index: payload.wall_index, load_case_index: payload.load_case_index, index: payload.index, new_force_n: base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated[payload.index].force_n })] }
}
