use super::RemoveOpening;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::insert_opening;
pub fn inverse(payload: &RemoveOpening, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() { Vec::new() } else { vec![En1996Mutation::InsertOpening(insert_opening::InsertOpening { wall_index: payload.wall_index, index: payload.index, opening: base.walls[payload.wall_index].openings[payload.index].clone() })] }
}
