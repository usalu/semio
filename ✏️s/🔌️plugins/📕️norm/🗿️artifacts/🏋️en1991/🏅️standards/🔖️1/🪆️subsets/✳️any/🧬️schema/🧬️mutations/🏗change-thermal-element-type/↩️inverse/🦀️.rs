//! Inverse for `change-thermal-element-type`.
use super::ChangeThermalElementType;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeThermalElementType, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeThermalElementType(ChangeThermalElementType { new_thermal_element_type: base.thermal_element_type.clone() })]
}
