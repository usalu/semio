//! Inverse for `insert-tank`.
use super::InsertTank;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_tank;

pub fn inverse(payload: &InsertTank, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveTank(remove_tank::RemoveTank { index: payload.index.unwrap_or(usize::MAX).min(base.tanks.len()) })]

    })())
}
