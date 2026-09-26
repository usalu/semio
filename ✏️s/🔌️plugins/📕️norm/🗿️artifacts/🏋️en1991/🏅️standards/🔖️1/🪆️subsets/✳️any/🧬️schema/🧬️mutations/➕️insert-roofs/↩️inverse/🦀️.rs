//! Inverse for `insert-roofs`.
use super::InsertRoofs;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertRoofs, _base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::RemoveRoofs(crate::mutations::remove_roofs::RemoveRoofs { index: payload.index })]
}
