use super::RemoveMember; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &RemoveMember, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1990Mutation::ChangeMembers(crate::standards::v1::subsets::any::schema::mutations::change_members::ChangeMembers { new_members: base.members.clone() })]

    })())
}
