//! Inverse for `insert-floors`.
use super::InsertFloors;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertFloors, _base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::RemoveFloors(crate::mutations::remove_floors::RemoveFloors { index: payload.index })]

    })())
}
