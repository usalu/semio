//! Inverse for `insert-tank`.
use super::InsertTank;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertTank, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
