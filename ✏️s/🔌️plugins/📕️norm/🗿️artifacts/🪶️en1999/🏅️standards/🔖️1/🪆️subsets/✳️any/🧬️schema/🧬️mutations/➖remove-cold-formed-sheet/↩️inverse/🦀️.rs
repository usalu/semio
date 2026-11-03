//! ↩️ `remove-cold-formed-sheet` inverse.

use crate::remove_cold_formed_sheet::RemoveColdFormedSheet;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_cold_formed_sheet;

pub fn inverse<Op>(payload: &RemoveColdFormedSheet, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_cold_formed_sheet::InsertColdFormedSheet> {
    Ok((|| {
    if let Some((index, row)) = base.cold_formed.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_cold_formed_sheet::InsertColdFormedSheet { index: index as u32, cold_formed_sheet: row.clone() })]
    } else { Vec::new() }

    })())
}
