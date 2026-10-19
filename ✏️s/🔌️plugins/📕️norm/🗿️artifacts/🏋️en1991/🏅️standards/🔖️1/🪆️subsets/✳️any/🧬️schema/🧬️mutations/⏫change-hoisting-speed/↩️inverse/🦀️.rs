//! ↩️ Inverse for `change-hoisting-speed`.
use crate::change_hoisting_speed::ChangeHoistingSpeed;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeHoistingSpeed, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_hoisting_speed::ChangeHoistingSpeed> {
    Ok((|| {
    vec![Op::from(ChangeHoistingSpeed { new_hoisting_speed: base.hoisting_speed })]

    })())
}
