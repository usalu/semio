use super::ChangeUnitLength;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeUnitLength, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeUnitLength(ChangeUnitLength { index: payload.index, new_unit_length_m: base.walls[payload.index].unit_length_m })] }
}
