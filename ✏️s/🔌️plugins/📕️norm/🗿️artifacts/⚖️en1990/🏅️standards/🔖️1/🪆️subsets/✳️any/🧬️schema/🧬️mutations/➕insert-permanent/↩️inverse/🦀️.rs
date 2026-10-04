use super::InsertPermanent; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &InsertPermanent, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1990Mutation::ChangePermanents(crate::standards::v1::subsets::any::schema::mutations::change_permanents::ChangePermanents { new_permanents: base.permanents.clone() })]

    })())
}
