//! Inverse for `change-thermal-bridge-type`.
use super::ChangeThermalBridgeType;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeThermalBridgeType, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeThermalBridgeType(ChangeThermalBridgeType { new_thermal_bridge_type: base.thermal_bridge_type })]
}
