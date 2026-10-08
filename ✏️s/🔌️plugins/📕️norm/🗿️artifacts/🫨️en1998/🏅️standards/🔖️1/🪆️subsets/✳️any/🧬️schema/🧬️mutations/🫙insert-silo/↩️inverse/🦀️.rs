//! Inverse for `insert-silo`.
use super::InsertSilo;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_silo;

pub fn inverse(payload: &InsertSilo, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveSilo(remove_silo::RemoveSilo { index: payload.index.unwrap_or(usize::MAX).min(base.silos.len()) })]

    })())
}
