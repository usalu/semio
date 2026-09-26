use super::ChangeUnitWidth;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeUnitWidth, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeUnitWidth(ChangeUnitWidth { index: payload.index, new_unit_width_m: base.walls[payload.index].unit_width_m })] }
}
