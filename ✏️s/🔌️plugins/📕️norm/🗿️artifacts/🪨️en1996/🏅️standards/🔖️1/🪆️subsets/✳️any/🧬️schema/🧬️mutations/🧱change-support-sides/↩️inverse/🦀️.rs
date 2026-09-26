use super::ChangeSupportSides;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeSupportSides, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeSupportSides(ChangeSupportSides { index: payload.index, new_support_sides: base.walls[payload.index].support_sides })] }
}
