//! 🏋️ `change-variables` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use crate::change_variables::ChangeVariables;
use crate::insert_variable::InsertVariable;
use crate::remove_variable::RemoveVariable;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeVariables, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_variable::RemoveVariable> + From<crate::insert_variable::InsertVariable> {
    let removed = mutation.new_variables.iter().map(|_| Op::from(RemoveVariable { index: 0 }));
    let inserted = base.variables.iter().cloned().enumerate().map(|(index, item)| Op::from(InsertVariable { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
