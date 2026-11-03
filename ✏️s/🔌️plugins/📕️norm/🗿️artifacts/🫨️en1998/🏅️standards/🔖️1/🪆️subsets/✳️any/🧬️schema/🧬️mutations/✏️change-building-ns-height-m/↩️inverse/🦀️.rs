//! ↩️ `change-building-ns-height-m` inverse: restores the previous `nonstructural_height_m`.

use crate::change_building_ns_height_m::ChangeBuildingNsHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingNsHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ns_height_m::ChangeBuildingNsHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingNsHeightM, base: &En1998Snapshot) -> Option<ChangeBuildingNsHeightM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingNsHeightM { id: payload.id.clone(), new_nonstructural_height_m: r0.nonstructural_height_m })
}
