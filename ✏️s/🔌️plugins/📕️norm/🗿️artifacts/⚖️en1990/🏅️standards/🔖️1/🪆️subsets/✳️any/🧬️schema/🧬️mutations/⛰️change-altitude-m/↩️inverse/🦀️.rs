//! ↩️ `change-altitude-m` inverse.

use crate::change_altitude_m::ChangeAltitudeM;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeAltitudeM, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_altitude_m::ChangeAltitudeM> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeAltitudeM { new_altitude_m: base.altitude_m })]

    })())
}
