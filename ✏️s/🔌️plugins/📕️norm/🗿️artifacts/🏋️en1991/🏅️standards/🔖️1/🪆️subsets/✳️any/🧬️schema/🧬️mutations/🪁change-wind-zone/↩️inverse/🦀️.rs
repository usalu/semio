//! ↩️ Inverse for `change-wind-zone`.
use crate::change_wind_zone::ChangeWindZone;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeWindZone, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_zone::ChangeWindZone> {
    Ok((|| {
    vec![Op::from(ChangeWindZone { new_wind_zone: base.wind_zone })]

    })())
}
