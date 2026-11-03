//! ↩️ `change-water-flow-length` inverse: restores the previous `flow_length`.

use crate::change_water_flow_length::ChangeWaterFlowLength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterFlowLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_flow_length::ChangeWaterFlowLength> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterFlowLength { id: payload.id.clone(), new_flow_length: row.flow_length })],
        None => Vec::new(),
    })
}
