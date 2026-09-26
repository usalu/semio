//! Inverse for `insert-slab`.
use super::InsertSlab;
use crate::artifact_schema::mutations::remove_slab::RemoveSlab;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &InsertSlab, _base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::RemoveSlab(RemoveSlab { index: payload.index })]
}
