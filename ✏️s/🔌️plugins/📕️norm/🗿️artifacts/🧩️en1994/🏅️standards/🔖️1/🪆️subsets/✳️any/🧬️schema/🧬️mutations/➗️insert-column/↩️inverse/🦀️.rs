//! Inverse for `insert-column`.
use super::InsertColumn;
use crate::artifact_schema::mutations::remove_column::RemoveColumn;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &InsertColumn, _base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::RemoveColumn(RemoveColumn { index: payload.index })]
}
