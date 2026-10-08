//! 🏋️ `change-variables` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangeVariables;
use crate::mutations::insert_variable::InsertVariable;
use crate::mutations::remove_variable::RemoveVariable;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangeVariables, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = mutation.new_variables.iter().map(|_| En1990Mutation::RemoveVariable(RemoveVariable { index: 0 }));
    let inserted = base.variables.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertVariable(InsertVariable { index, item }));
    Ok(removed.chain(inserted).rev().collect())
}
