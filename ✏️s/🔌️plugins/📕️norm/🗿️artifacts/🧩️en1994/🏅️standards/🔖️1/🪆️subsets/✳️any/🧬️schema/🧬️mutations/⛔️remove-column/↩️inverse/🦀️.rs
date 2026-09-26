//! Inverse for `remove-column`.
use super::RemoveColumn;
use crate::artifact_schema::mutations::insert_column::InsertColumn;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &RemoveColumn, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(column) = base.columns.get(payload.index).cloned() else { return Vec::new(); };
    vec![En1994Mutation::InsertColumn(InsertColumn { index: payload.index, column })]
}
