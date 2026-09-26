//! Diff for `change-thermal-bridge-type`.
use super::ChangeThermalBridgeType;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeThermalBridgeType, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.thermal_bridge_type == payload.new_thermal_bridge_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_bridge_type: Some(payload.new_thermal_bridge_type), ..Default::default() })
}
