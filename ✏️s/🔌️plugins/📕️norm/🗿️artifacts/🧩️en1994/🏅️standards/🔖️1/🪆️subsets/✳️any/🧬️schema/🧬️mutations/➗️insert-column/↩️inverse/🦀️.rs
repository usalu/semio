//! Inverse for `insert-column`.
use crate::insert_column::InsertColumn;
use crate::remove_column::RemoveColumn;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &InsertColumn, _base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_column::RemoveColumn> {
    Ok((|| {
    vec![Op::from(RemoveColumn { index: payload.index })]

    })())
}
