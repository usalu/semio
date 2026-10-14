//! 🔺️ Diff for `change-thermal-element-type`.
use crate::change_thermal_element_type::ChangeThermalElementType;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeThermalElementType, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.thermal_element_type == payload.new_thermal_element_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_element_type: Some(payload.new_thermal_element_type.clone()), ..Default::default() })
}
