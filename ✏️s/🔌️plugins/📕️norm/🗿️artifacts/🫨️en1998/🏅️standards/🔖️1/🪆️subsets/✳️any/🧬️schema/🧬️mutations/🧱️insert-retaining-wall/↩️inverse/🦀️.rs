//! Inverse for `insert-retaining-wall`.
use super::InsertRetainingWall;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_retaining_wall;

pub fn inverse(payload: &InsertRetainingWall, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::RemoveRetainingWall(remove_retaining_wall::RemoveRetainingWall { index: payload.index.min(base.retaining_walls.len()) })]
}
