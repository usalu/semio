//! ↩️ `insert-cold-formed-sheet` inverse.

use crate::insert_cold_formed_sheet::InsertColdFormedSheet;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_cold_formed_sheet;

pub fn inverse<Op>(payload: &InsertColdFormedSheet, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_cold_formed_sheet::RemoveColdFormedSheet> {
    Ok((|| {
    vec![Op::from(remove_cold_formed_sheet::RemoveColdFormedSheet { id: payload.cold_formed_sheet.id.clone() })]

    })())
}
