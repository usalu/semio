//! Inverse for `remove-foundation`.
use super::RemoveFoundation;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_foundation;

pub fn inverse(payload: &RemoveFoundation, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.foundations.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertFoundation(insert_foundation::InsertFoundation { index: Some(payload.index), foundation: item.clone() })],
        None => Vec::new(),
    }

    })())
}
