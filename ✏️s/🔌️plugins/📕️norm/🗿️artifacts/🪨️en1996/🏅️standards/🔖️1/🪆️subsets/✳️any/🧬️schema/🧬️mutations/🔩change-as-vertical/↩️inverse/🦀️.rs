use super::ChangeAsVertical;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeAsVertical, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeAsVertical(ChangeAsVertical { index: payload.index, new_as_vertical_m2: base.walls[payload.index].as_vertical_m2 })] }
}
