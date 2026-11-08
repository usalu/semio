//! ↩️ `change-critical-region-clear-length-m` inverse: restores the previous `clear_length_m`.

use crate::change_critical_region_clear_length_m::ChangeCriticalRegionClearLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionClearLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_clear_length_m::ChangeCriticalRegionClearLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionClearLengthM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionClearLengthM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionClearLengthM { id: payload.id.clone(), new_clear_length_m: r0.clear_length_m })
}
