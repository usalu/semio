//! 📥 `insert-variable` inverse — removes the row at the position the insert landed on.

use crate::insert_variable::InsertVariable;
use crate::remove_variable::RemoveVariable;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertVariable, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_variable::RemoveVariable> {
    Ok(vec![Op::from(RemoveVariable { index: payload.index.unwrap_or(usize::MAX).min(base.variables.len()) })])
}
