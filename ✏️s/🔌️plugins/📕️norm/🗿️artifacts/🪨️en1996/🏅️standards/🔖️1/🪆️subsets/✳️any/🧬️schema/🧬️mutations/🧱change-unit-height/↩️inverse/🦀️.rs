use super::ChangeUnitHeight;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeUnitHeight, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeUnitHeight(ChangeUnitHeight { index: payload.index, new_unit_height_m: base.walls[payload.index].unit_height_m })] }
}
