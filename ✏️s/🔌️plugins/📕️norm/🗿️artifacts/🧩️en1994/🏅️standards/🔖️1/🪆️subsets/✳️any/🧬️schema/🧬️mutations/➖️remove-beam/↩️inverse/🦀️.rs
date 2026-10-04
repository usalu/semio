//! Inverse for `remove-beam`.
use super::RemoveBeam;
use crate::artifact_schema::mutations::insert_beam::InsertBeam;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &RemoveBeam, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index).cloned() else { return Vec::new(); };
    vec![En1994Mutation::InsertBeam(InsertBeam { index: payload.index, beam })]

    })())
}
