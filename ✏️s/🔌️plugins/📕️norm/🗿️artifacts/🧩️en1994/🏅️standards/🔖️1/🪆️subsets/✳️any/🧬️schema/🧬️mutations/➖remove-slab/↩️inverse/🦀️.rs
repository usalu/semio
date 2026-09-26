//! Inverse for `remove-slab`.
use super::RemoveSlab;
use crate::artifact_schema::mutations::insert_slab::InsertSlab;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &RemoveSlab, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(slab) = base.slabs.get(payload.index).cloned() else { return Vec::new(); };
    vec![En1994Mutation::InsertSlab(InsertSlab { index: payload.index, slab })]
}
