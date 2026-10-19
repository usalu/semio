//! ↩️ Inverse for `change-air-density`.
use crate::change_air_density::ChangeAirDensity;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAirDensity, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_air_density::ChangeAirDensity> {
    Ok((|| {
    vec![Op::from(ChangeAirDensity { new_air_density: base.air_density })]

    })())
}
