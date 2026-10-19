//! ↩️ Inverse for `change-thermal-bridge-type`.
use crate::change_thermal_bridge_type::ChangeThermalBridgeType;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeThermalBridgeType, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_bridge_type::ChangeThermalBridgeType> {
    Ok((|| {
    vec![Op::from(ChangeThermalBridgeType { new_thermal_bridge_type: base.thermal_bridge_type })]

    })())
}
