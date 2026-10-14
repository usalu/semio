//! Inverse for `change-thermal-element-type`.
use crate::change_thermal_element_type::ChangeThermalElementType;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeThermalElementType, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_element_type::ChangeThermalElementType> {
    Ok((|| {
    vec![Op::from(ChangeThermalElementType { new_thermal_element_type: base.thermal_element_type.clone() })]

    })())
}
