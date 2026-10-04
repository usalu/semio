//! Inverse for `insert-foundation`.
use super::InsertFoundation;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_foundation;

pub fn inverse(payload: &InsertFoundation, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveFoundation(remove_foundation::RemoveFoundation { index: payload.index.min(base.foundations.len()) })]

    })())
}
