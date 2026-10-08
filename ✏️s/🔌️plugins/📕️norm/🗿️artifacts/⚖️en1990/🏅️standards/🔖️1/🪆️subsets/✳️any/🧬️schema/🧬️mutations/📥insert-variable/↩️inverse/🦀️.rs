//! 📥 `insert-variable` inverse — removes the row at the position the insert landed on.

use super::InsertVariable;
use crate::mutations::remove_variable::RemoveVariable;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &InsertVariable, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(vec![En1990Mutation::RemoveVariable(RemoveVariable { index: payload.index.unwrap_or(usize::MAX).min(base.variables.len()) })])
}
