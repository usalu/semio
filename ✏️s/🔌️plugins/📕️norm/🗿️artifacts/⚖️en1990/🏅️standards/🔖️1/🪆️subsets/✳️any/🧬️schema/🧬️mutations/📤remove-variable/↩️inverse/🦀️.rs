use super::RemoveVariable; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &RemoveVariable, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1990Mutation::ChangeVariables(crate::standards::v1::subsets::any::schema::mutations::change_variables::ChangeVariables { new_variables: base.variables.clone() })]

    })())
}
