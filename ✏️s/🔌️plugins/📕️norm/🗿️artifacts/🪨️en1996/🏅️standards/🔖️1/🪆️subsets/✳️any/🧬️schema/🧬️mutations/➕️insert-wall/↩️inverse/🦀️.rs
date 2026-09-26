use super::InsertWall;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::remove_wall;
pub fn inverse(payload: &InsertWall, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::RemoveWall(remove_wall::RemoveWall { index: payload.index })]
}
