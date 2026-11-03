//! ↩️ `change-snow-guard-roof-id` inverse: restores the previous `roof_id`.

use crate::change_snow_guard_roof_id::ChangeSnowGuardRoofId;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSnowGuardRoofId, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_snow_guard_roof_id::ChangeSnowGuardRoofId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSnowGuardRoofId, base: &En1991Snapshot) -> Option<ChangeSnowGuardRoofId> {
    let r0 = base.snow_guards.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSnowGuardRoofId { id: payload.id.clone(), new_roof_id: r0.roof_id.clone() })
}
