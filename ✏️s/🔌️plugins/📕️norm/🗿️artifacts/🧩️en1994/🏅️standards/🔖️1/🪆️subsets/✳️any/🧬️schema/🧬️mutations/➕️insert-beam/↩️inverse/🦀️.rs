//! Inverse for `insert-beam`.
use super::InsertBeam;
use crate::artifact_schema::mutations::remove_beam::RemoveBeam;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &InsertBeam, _base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1994Mutation::RemoveBeam(RemoveBeam { index: payload.index })]

    })())
}
