use super::ChangeWallHeight;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeWallHeight, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeWallHeight(ChangeWallHeight { index: payload.index, new_height_m: base.walls[payload.index].height_m })] }
}
