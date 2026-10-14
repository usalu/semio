//! 📤 `remove-variable` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_variable::RemoveVariable;
use crate::insert_variable::InsertVariable;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveVariable, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_variable::InsertVariable> {
    Ok(base.variables.get(payload.index).map(|item| vec![Op::from(InsertVariable { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
