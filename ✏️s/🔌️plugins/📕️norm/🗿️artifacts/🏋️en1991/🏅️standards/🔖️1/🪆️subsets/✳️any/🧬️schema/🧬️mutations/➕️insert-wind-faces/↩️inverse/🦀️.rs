//! Inverse for `insert-wind-faces`.
use super::InsertWindFaces;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertWindFaces, _base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::RemoveWindFaces(crate::mutations::remove_wind_faces::RemoveWindFaces { index: payload.index })]

    })())
}
