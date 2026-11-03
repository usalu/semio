//! ↩️ `change-foundation-supported-building-id` inverse: restores the previous `supported_building_id`.

use crate::change_foundation_supported_building_id::ChangeFoundationSupportedBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationSupportedBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_supported_building_id::ChangeFoundationSupportedBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationSupportedBuildingId, base: &En1998Snapshot) -> Option<ChangeFoundationSupportedBuildingId> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationSupportedBuildingId { id: payload.id.clone(), new_supported_building_id: r0.supported_building_id.clone() })
}
