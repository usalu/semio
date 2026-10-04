//! Inverse for `insert-building`.
use super::InsertBuilding;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_building;

pub fn inverse(payload: &InsertBuilding, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveBuilding(remove_building::RemoveBuilding { index: payload.index.min(base.buildings.len()) })]

    })())
}
