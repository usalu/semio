//! ↩️ `remove-transverse-stiffener` inverse.

use crate::remove_transverse_stiffener::RemoveTransverseStiffener;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_transverse_stiffener;

pub fn inverse<Op>(payload: &RemoveTransverseStiffener, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_transverse_stiffener::InsertTransverseStiffener> {
    Ok((|| {
    if let Some((index, row)) = base.transverse_stiffeners.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_transverse_stiffener::InsertTransverseStiffener { index: index as u32, transverse_stiffener: row.clone() })]
    } else { Vec::new() }

    })())
}
