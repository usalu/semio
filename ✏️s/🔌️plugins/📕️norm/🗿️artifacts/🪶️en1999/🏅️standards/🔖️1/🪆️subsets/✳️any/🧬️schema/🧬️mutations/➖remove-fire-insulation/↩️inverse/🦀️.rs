//! ↩️ `remove-fire-insulation` inverse.

use crate::remove_fire_insulation::RemoveFireInsulation;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_fire_insulation;

pub fn inverse<Op>(payload: &RemoveFireInsulation, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fire_insulation::InsertFireInsulation> {
    Ok((|| {
    if let Some((index, row)) = base.fire_insulations.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_fire_insulation::InsertFireInsulation { index: index as u32, fire_insulation: row.clone() })]
    } else { Vec::new() }

    })())
}
