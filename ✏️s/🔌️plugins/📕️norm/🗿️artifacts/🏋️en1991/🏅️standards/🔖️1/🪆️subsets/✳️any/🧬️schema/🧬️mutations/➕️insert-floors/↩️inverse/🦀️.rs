//! Inverse for `insert-floors`.
use super::InsertFloors;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertFloors, _base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::RemoveFloors(crate::mutations::remove_floors::RemoveFloors { index: payload.index })]
}
