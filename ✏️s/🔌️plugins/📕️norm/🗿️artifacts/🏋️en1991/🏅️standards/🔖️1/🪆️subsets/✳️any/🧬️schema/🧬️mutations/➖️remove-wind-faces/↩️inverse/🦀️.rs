//! Inverse for `remove-wind-faces`.
use super::RemoveWindFaces;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &RemoveWindFaces, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.wind_faces.len() { return Vec::new(); }
    let item = base.wind_faces[payload.index].clone();
    vec![En1991Mutation::InsertWindFaces(crate::mutations::insert_wind_faces::InsertWindFaces { index: payload.index, item })]

    })())
}
