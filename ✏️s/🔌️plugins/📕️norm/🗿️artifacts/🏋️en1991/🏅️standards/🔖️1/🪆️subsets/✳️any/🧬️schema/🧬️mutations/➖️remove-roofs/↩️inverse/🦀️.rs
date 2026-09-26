//! Inverse for `remove-roofs`.
use super::RemoveRoofs;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &RemoveRoofs, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.roofs.len() { return Vec::new(); }
    let item = base.roofs[payload.index].clone();
    vec![En1991Mutation::InsertRoofs(crate::mutations::insert_roofs::InsertRoofs { index: payload.index, item })]
}
