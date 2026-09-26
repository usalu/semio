use super::ChangeIsBasement;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeIsBasement, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else {
        vec![En1996Mutation::ChangeIsBasement(ChangeIsBasement { index: payload.index, new_is_basement: base.walls[payload.index].is_basement })]
    }
}
