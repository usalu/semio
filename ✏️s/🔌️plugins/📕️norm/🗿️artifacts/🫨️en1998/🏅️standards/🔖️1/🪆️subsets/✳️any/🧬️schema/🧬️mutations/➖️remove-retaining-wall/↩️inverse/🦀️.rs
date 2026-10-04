//! Inverse for `remove-retaining-wall`.
use super::RemoveRetainingWall;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_retaining_wall;

pub fn inverse(payload: &RemoveRetainingWall, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.retaining_walls.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertRetainingWall(insert_retaining_wall::InsertRetainingWall { index: payload.index, wall: item.clone() })],
        None => Vec::new(),
    }

    })())
}
