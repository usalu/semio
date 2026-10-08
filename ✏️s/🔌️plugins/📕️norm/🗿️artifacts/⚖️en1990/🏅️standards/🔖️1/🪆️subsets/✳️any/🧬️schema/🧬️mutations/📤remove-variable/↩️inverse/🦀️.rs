//! 📤 `remove-variable` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use super::RemoveVariable;
use crate::mutations::insert_variable::InsertVariable;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &RemoveVariable, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(base.variables.get(payload.index).map(|item| vec![En1990Mutation::InsertVariable(InsertVariable { index: payload.index, item: item.clone() })]).unwrap_or_default())
}
