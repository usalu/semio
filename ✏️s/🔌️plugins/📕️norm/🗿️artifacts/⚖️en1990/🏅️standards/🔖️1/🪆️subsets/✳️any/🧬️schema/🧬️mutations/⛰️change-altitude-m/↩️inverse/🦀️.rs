//! ↩️ `change-altitude-m` inverse.

use super::ChangeAltitudeM;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeAltitudeM, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeAltitudeM(ChangeAltitudeM { new_altitude_m: base.altitude_m })]

    })())
}
