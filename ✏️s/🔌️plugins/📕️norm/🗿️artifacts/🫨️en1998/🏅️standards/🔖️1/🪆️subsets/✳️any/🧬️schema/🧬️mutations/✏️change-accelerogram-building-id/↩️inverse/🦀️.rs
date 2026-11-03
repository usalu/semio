//! ↩️ `change-accelerogram-building-id` inverse: restores the previous `building_id`.

use crate::change_accelerogram_building_id::ChangeAccelerogramBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAccelerogramBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accelerogram_building_id::ChangeAccelerogramBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccelerogramBuildingId, base: &En1998Snapshot) -> Option<ChangeAccelerogramBuildingId> {
    let r0 = base.accelerograms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAccelerogramBuildingId { id: payload.id.clone(), new_building_id: r0.building_id.clone() })
}
