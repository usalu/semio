use super::ChangeOpeningHeight;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeOpeningHeight, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() { Vec::new() } else { vec![En1996Mutation::ChangeOpeningHeight(ChangeOpeningHeight { wall_index: payload.wall_index, index: payload.index, new_height_m: base.walls[payload.wall_index].openings[payload.index].height_m })] }
}
