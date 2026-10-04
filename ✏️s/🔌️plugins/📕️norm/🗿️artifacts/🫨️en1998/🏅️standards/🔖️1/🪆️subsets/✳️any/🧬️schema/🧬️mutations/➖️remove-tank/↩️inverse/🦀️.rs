//! Inverse for `remove-tank`.
use super::RemoveTank;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_tank;

pub fn inverse(payload: &RemoveTank, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.tanks.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertTank(insert_tank::InsertTank { index: payload.index, tank: item.clone() })],
        None => Vec::new(),
    }

    })())
}
