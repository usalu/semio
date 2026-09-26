//! Inverse for `insert-silo`.
use super::InsertSilo;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertSilo, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
