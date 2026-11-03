//! ↩️ `change-water-head-difference` inverse: restores the previous `head_difference`.

use crate::change_water_head_difference::ChangeWaterHeadDifference;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterHeadDifference, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_head_difference::ChangeWaterHeadDifference> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterHeadDifference { id: payload.id.clone(), new_head_difference: row.head_difference })],
        None => Vec::new(),
    })
}
