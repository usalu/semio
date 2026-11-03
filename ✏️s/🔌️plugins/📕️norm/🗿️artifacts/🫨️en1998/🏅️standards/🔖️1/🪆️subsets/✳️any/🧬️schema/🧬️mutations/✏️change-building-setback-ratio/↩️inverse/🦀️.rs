//! ↩️ `change-building-setback-ratio` inverse: restores the previous `setback_ratio`.

use crate::change_building_setback_ratio::ChangeBuildingSetbackRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingSetbackRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_setback_ratio::ChangeBuildingSetbackRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingSetbackRatio, base: &En1998Snapshot) -> Option<ChangeBuildingSetbackRatio> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingSetbackRatio { id: payload.id.clone(), new_setback_ratio: r0.setback_ratio })
}
