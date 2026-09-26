//! Inverse for `remove-floors`.
use super::RemoveFloors;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &RemoveFloors, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.floors.len() { return Vec::new(); }
    let item = base.floors[payload.index].clone();
    vec![En1991Mutation::InsertFloors(crate::mutations::insert_floors::InsertFloors { index: payload.index, item })]
}
