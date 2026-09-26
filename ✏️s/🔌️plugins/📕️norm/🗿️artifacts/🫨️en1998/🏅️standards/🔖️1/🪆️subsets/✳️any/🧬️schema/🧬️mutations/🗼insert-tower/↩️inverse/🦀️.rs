//! Inverse for `insert-tower`.
use super::InsertTower;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertTower, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
