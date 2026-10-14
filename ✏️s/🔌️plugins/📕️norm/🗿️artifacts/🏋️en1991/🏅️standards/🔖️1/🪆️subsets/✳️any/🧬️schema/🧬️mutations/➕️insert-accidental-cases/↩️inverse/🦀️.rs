//! Inverse for `insert-accidental-cases`.
use crate::insert_accidental_cases::InsertAccidentalCases;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &InsertAccidentalCases, _base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_accidental_cases::RemoveAccidentalCases> {
    Ok((|| {
    vec![Op::from(crate::remove_accidental_cases::RemoveAccidentalCases { index: payload.index })]

    })())
}
