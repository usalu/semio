//! Inverse for `remove-silo`.
use super::RemoveSilo;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_silo;

pub fn inverse(payload: &RemoveSilo, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.silos.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertSilo(insert_silo::InsertSilo { index: payload.index, silo: item.clone() })],
        None => Vec::new(),
    }

    })())
}
