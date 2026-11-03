//! ↩️ `remove-placement` inverse.

use crate::remove_placement::RemovePlacement;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_placement;

pub fn inverse<Op>(payload: &RemovePlacement, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok((|| {
    if let Some((index, row)) = base.placements.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_placement::InsertPlacement { index: index as u32, placement: row.clone() })]
    } else { Vec::new() }

    })())
}
