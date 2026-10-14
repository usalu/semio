//! Inverse for `change-fire-occupancy`.
use crate::change_fire_occupancy::ChangeFireOccupancy;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireOccupancy, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_occupancy::ChangeFireOccupancy> {
    Ok((|| {
    vec![Op::from(ChangeFireOccupancy { new_fire_occupancy: base.fire_occupancy.clone() })]

    })())
}
