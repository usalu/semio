//! ↩️ `change-altitude-m` inverse.

use crate::change_altitude_m::ChangeAltitudeM;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeAltitudeM, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_altitude_m::ChangeAltitudeM> {
    Ok(match previous(mutation, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(_mutation: &ChangeAltitudeM, base: &En1990Snapshot) -> Option<crate::change_altitude_m::ChangeAltitudeM> {
    Some(ChangeAltitudeM { new_altitude_m: base.altitude_m })
}
