//! ↩️ Inverse for `change-initial-temperature`.
use crate::change_initial_temperature::ChangeInitialTemperature;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeInitialTemperature, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_initial_temperature::ChangeInitialTemperature> {
    Ok((|| {
    vec![Op::from(ChangeInitialTemperature { new_t_0: base.t_0 })]

    })())
}
