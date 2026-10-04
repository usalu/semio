//! Inverse for `change-fire-thermal-inertia`.
use super::ChangeFireThermalInertia;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireThermalInertia, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeFireThermalInertia(ChangeFireThermalInertia { new_fire_thermal_inertia: base.fire_thermal_inertia })]

    })())
}
