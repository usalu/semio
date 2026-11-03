//! ↩️ `change-building-storey-stiffness-ratio` inverse: restores the previous `storey_stiffness_ratio`.

use crate::change_building_storey_stiffness_ratio::ChangeBuildingStoreyStiffnessRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingStoreyStiffnessRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_storey_stiffness_ratio::ChangeBuildingStoreyStiffnessRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingStoreyStiffnessRatio, base: &En1998Snapshot) -> Option<ChangeBuildingStoreyStiffnessRatio> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingStoreyStiffnessRatio { id: payload.id.clone(), new_storey_stiffness_ratio: r0.storey_stiffness_ratio })
}
