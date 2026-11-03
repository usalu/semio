//! ↩️ `change-storey-building-height` inverse: restores the previous `building_height`.

use crate::change_storey_building_height::ChangeStoreyBuildingHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyBuildingHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_building_height::ChangeStoreyBuildingHeight> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyBuildingHeight { id: payload.id.clone(), new_building_height: row.building_height })],
        None => Vec::new(),
    })
}
