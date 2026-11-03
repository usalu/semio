//! ↩️ `change-diaphragm-building-id` inverse: restores the previous `building_id`.

use crate::change_diaphragm_building_id::ChangeDiaphragmBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_building_id::ChangeDiaphragmBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmBuildingId, base: &En1998Snapshot) -> Option<ChangeDiaphragmBuildingId> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmBuildingId { id: payload.id.clone(), new_building_id: r0.building_id.clone() })
}
