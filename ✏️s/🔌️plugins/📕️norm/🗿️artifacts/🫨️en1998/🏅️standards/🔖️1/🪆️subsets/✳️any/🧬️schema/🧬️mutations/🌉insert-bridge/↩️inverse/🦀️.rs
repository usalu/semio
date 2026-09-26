//! Inverse for `insert-bridge`.
use super::InsertBridge;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertBridge, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
