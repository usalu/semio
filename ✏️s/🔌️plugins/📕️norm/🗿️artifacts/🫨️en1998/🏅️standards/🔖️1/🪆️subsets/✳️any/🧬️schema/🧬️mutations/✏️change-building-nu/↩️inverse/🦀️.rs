//! ↩️ `change-building-nu` inverse: restores the previous `nu`.

use crate::change_building_nu::ChangeBuildingNu;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingNu, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_nu::ChangeBuildingNu> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingNu, base: &En1998Snapshot) -> Option<ChangeBuildingNu> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingNu { id: payload.id.clone(), new_nu: r0.nu })
}
