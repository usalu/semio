use super::ChangeAsHorizontal;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeAsHorizontal, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeAsHorizontal(ChangeAsHorizontal { index: payload.index, new_as_horizontal_m2: base.walls[payload.index].as_horizontal_m2 })] }
}
