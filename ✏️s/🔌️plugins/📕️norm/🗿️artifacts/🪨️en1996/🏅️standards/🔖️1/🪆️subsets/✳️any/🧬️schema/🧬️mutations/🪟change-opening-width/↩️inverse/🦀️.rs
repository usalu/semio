use super::ChangeOpeningWidth;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeOpeningWidth, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() { Vec::new() } else { vec![En1996Mutation::ChangeOpeningWidth(ChangeOpeningWidth { wall_index: payload.wall_index, index: payload.index, new_width_m: base.walls[payload.wall_index].openings[payload.index].width_m })] }
}
