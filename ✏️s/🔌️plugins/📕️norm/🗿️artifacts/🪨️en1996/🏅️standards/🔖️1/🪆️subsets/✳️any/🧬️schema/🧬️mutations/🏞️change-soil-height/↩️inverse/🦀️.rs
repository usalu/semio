//! ↩️ `change-soil-height` inverse: restores the previous `soil_height_m`.

use crate::change_soil_height::ChangeSoilHeight;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilHeight, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_height::ChangeSoilHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilHeight, base: &En1996Snapshot) -> Option<ChangeSoilHeight> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilHeight { id: payload.id.clone(), new_soil_height_m: r0.soil_height_m })
}
