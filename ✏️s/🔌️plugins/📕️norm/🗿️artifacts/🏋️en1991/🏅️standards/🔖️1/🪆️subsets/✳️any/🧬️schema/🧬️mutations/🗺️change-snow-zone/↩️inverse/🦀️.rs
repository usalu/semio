//! Inverse for `change-snow-zone`.
use crate::change_snow_zone::ChangeSnowZone;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSnowZone, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_snow_zone::ChangeSnowZone> {
    Ok((|| {
    vec![Op::from(ChangeSnowZone { new_snow_zone: base.snow_zone.clone() })]

    })())
}
