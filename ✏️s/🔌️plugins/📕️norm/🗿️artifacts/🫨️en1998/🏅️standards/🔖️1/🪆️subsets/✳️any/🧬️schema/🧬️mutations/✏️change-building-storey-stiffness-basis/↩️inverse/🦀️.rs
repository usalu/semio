//! ↩️ `change-building-storey-stiffness-basis` inverse: restores the previous `storey_stiffness_basis`.

use crate::change_building_storey_stiffness_basis::ChangeBuildingStoreyStiffnessBasis;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingStoreyStiffnessBasis, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_storey_stiffness_basis::ChangeBuildingStoreyStiffnessBasis> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingStoreyStiffnessBasis, base: &En1998Snapshot) -> Option<ChangeBuildingStoreyStiffnessBasis> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingStoreyStiffnessBasis { id: payload.id.clone(), new_storey_stiffness_basis: r0.storey_stiffness_basis })
}
