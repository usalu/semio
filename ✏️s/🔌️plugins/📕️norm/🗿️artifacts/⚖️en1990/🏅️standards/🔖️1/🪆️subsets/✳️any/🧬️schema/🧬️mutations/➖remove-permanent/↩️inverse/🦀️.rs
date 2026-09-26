use super::RemovePermanent; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &RemovePermanent, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangePermanents(crate::standards::v1::subsets::any::schema::mutations::change_permanents::ChangePermanents { new_permanents: base.permanents.clone() })]
}
