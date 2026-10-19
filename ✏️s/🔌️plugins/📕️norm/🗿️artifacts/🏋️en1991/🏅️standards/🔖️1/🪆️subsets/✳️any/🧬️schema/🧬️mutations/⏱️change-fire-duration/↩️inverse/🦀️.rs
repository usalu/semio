//! ↩️ Inverse for `change-fire-duration`.
use crate::change_fire_duration::ChangeFireDuration;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireDuration, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_duration::ChangeFireDuration> {
    Ok((|| {
    vec![Op::from(ChangeFireDuration { new_fire_duration: base.fire_duration })]

    })())
}
