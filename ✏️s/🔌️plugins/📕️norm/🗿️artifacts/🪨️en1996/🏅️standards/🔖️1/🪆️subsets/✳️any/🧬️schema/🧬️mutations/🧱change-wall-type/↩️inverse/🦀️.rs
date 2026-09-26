use super::ChangeWallType;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeWallType, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeWallType(ChangeWallType { index: payload.index, new_wall_type: base.walls[payload.index].wall_type })] }
}
