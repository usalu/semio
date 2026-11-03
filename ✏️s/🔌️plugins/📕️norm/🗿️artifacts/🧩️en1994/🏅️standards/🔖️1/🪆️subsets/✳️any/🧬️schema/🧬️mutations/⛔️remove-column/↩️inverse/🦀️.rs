//! ↩️ Inverse for `remove-column`.
use crate::remove_column::RemoveColumn;
use crate::insert_column::InsertColumn;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &RemoveColumn, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_column::InsertColumn> {
    Ok((|| {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(column) = base.columns.get(index).cloned() else { return Vec::new(); };
    vec![Op::from(InsertColumn { index: index, column })]

    })())
}
