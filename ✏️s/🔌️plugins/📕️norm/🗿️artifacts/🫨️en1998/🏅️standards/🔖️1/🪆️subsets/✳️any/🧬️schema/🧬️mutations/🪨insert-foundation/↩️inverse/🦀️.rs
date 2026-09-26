//! Inverse for `insert-foundation`.
use super::InsertFoundation;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertFoundation, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
