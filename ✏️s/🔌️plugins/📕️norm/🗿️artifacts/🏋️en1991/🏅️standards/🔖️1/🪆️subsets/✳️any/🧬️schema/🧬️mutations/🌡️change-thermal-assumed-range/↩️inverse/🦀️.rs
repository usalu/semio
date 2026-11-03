//! ↩️ `change-thermal-assumed-range` inverse: restores the previous `assumed_range`.

use crate::change_thermal_assumed_range::ChangeThermalAssumedRange;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalAssumedRange, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_assumed_range::ChangeThermalAssumedRange> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeThermalAssumedRange, base: &En1991Snapshot) -> Option<ChangeThermalAssumedRange> {
    let r0 = base.thermal_sections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeThermalAssumedRange { id: payload.id.clone(), new_assumed_range: r0.assumed_range })
}
