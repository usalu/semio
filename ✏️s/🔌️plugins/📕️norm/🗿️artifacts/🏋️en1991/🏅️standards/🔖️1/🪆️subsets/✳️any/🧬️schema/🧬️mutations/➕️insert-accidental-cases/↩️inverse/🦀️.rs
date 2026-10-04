//! Inverse for `insert-accidental-cases`.
use super::InsertAccidentalCases;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertAccidentalCases, _base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::RemoveAccidentalCases(crate::mutations::remove_accidental_cases::RemoveAccidentalCases { index: payload.index })]

    })())
}
