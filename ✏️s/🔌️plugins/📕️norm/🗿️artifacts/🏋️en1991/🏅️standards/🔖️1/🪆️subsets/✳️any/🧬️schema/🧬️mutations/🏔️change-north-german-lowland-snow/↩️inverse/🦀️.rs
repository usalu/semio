//! ↩️ Inverse for `change-north-german-lowland-snow`.
use crate::change_north_german_lowland_snow::ChangeNorthGermanLowlandSnow;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeNorthGermanLowlandSnow, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_north_german_lowland_snow::ChangeNorthGermanLowlandSnow> {
    Ok((|| {
    vec![Op::from(ChangeNorthGermanLowlandSnow { new_north_german_lowland_snow: base.north_german_lowland_snow })]

    })())
}
