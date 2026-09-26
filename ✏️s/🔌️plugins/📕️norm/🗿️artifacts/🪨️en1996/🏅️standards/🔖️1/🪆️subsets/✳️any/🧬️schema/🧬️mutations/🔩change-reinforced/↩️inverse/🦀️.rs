use super::ChangeReinforced;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeReinforced, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeReinforced(ChangeReinforced { index: payload.index, new_reinforced: base.walls[payload.index].reinforced })] }
}
