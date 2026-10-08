//! Inverse for `remove-building`.
use super::RemoveBuilding;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_building;

pub fn inverse(payload: &RemoveBuilding, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.buildings.get(payload.index) {
        Some(b) => vec![En1998Mutation::InsertBuilding(insert_building::InsertBuilding { index: Some(payload.index), building: b.clone() })],
        None => Vec::new(),
    }

    })())
}
