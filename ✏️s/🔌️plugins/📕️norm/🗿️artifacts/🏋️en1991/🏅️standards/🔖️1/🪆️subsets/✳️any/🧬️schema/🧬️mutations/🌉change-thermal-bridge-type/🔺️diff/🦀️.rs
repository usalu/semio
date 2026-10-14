//! 🔺️ Diff for `change-thermal-bridge-type`.
use crate::change_thermal_bridge_type::ChangeThermalBridgeType;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeThermalBridgeType, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.thermal_bridge_type == payload.new_thermal_bridge_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_bridge_type: Some(payload.new_thermal_bridge_type), ..Default::default() })
}
