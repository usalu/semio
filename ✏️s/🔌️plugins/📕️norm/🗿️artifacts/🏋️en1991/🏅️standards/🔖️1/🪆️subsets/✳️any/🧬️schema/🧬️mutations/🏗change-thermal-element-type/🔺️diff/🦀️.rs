//! Diff for `change-thermal-element-type`.
use super::ChangeThermalElementType;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeThermalElementType, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.thermal_element_type == payload.new_thermal_element_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_element_type: Some(payload.new_thermal_element_type.clone()), ..Default::default() })
}
