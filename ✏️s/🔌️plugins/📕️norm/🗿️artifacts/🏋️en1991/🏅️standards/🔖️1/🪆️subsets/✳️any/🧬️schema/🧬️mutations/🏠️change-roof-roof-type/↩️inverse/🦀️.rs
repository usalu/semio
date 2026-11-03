//! ↩️ `change-roof-roof-type` inverse: restores the previous `roof_type`.

use crate::change_roof_roof_type::ChangeRoofRoofType;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofRoofType, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_roof_type::ChangeRoofRoofType> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofRoofType, base: &En1991Snapshot) -> Option<ChangeRoofRoofType> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofRoofType { id: payload.id.clone(), new_roof_type: r0.roof_type.clone() })
}
