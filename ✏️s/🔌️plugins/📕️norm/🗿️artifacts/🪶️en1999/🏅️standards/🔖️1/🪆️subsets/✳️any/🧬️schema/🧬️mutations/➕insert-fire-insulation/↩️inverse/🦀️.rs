//! ↩️ `insert-fire-insulation` inverse.

use crate::insert_fire_insulation::InsertFireInsulation;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_fire_insulation;

pub fn inverse<Op>(payload: &InsertFireInsulation, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fire_insulation::RemoveFireInsulation> {
    Ok((|| {
    vec![Op::from(remove_fire_insulation::RemoveFireInsulation { id: payload.fire_insulation.id.clone() })]

    })())
}
