//! Inverse for `insert-retaining-wall`.
use super::InsertRetainingWall;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertRetainingWall, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
