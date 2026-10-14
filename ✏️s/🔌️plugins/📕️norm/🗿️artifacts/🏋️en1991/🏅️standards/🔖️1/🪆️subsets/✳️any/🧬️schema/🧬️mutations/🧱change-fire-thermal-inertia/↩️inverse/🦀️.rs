//! Inverse for `change-fire-thermal-inertia`.
use crate::change_fire_thermal_inertia::ChangeFireThermalInertia;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireThermalInertia, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_thermal_inertia::ChangeFireThermalInertia> {
    Ok((|| {
    vec![Op::from(ChangeFireThermalInertia { new_fire_thermal_inertia: base.fire_thermal_inertia })]

    })())
}
