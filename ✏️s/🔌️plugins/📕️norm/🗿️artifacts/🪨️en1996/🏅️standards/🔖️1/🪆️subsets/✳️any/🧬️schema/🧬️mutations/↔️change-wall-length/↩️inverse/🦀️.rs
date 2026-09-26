use super::ChangeWallLength;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeWallLength, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeWallLength(ChangeWallLength { index: payload.index, new_length_m: base.walls[payload.index].length_m })] }
}
