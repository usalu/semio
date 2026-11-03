//! ↩️ `change-water-influence-radius` inverse: restores the previous `influence_radius`.

use crate::change_water_influence_radius::ChangeWaterInfluenceRadius;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterInfluenceRadius, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_influence_radius::ChangeWaterInfluenceRadius> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterInfluenceRadius { id: payload.id.clone(), new_influence_radius: row.influence_radius })],
        None => Vec::new(),
    })
}
