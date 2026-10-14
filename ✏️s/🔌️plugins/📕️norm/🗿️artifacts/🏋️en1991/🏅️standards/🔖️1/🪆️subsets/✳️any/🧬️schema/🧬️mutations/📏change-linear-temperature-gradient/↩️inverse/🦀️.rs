//! Inverse for `change-linear-temperature-gradient`.
use crate::change_linear_temperature_gradient::ChangeLinearTemperatureGradient;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeLinearTemperatureGradient, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_linear_temperature_gradient::ChangeLinearTemperatureGradient> {
    Ok((|| {
    vec![Op::from(ChangeLinearTemperatureGradient { new_delta_t_m: base.delta_t_m })]

    })())
}
