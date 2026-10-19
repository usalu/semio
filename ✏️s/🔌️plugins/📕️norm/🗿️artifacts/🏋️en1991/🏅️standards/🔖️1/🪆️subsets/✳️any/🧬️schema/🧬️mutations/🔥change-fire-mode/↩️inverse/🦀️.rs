//! ↩️ Inverse for `change-fire-mode`.
use crate::change_fire_mode::ChangeFireMode;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireMode, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_mode::ChangeFireMode> {
    Ok((|| {
    vec![Op::from(ChangeFireMode { new_fire_mode: base.fire_mode })]

    })())
}
