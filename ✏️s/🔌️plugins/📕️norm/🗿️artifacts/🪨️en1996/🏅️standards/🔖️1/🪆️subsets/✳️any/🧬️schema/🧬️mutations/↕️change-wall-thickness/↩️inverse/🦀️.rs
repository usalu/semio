use super::ChangeWallThickness;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeWallThickness, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeWallThickness(ChangeWallThickness { index: payload.index, new_thickness_m: base.walls[payload.index].thickness_m })] }
}
