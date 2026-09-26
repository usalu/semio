use super::RemoveMember; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &RemoveMember, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeMembers(crate::standards::v1::subsets::any::schema::mutations::change_members::ChangeMembers { new_members: base.members.clone() })]
}
