//! ↩️ `change-variables` inverse.

use super::ChangeVariables;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeVariables, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeVariables(ChangeVariables { new_variables: base.variables.clone() })]

    })())
}
