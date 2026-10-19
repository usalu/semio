//! ↩️ Inverse for `change-altitude`.
use crate::change_altitude::ChangeAltitude;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAltitude, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_altitude::ChangeAltitude> {
    Ok((|| {
    vec![Op::from(ChangeAltitude { new_altitude: base.altitude })]

    })())
}
