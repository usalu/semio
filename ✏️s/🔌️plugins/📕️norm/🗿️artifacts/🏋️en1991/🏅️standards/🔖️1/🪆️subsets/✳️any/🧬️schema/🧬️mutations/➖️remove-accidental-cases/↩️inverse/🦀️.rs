//! ↩️ Inverse for `remove-accidental-cases`.
use crate::remove_accidental_cases::RemoveAccidentalCases;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &RemoveAccidentalCases, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_accidental_cases::InsertAccidentalCases> {
    Ok((|| {
    if payload.index >= base.accidental_cases.len() { return Vec::new(); }
    let item = base.accidental_cases[payload.index].clone();
    vec![Op::from(crate::insert_accidental_cases::InsertAccidentalCases { index: payload.index, item })]

    })())
}
