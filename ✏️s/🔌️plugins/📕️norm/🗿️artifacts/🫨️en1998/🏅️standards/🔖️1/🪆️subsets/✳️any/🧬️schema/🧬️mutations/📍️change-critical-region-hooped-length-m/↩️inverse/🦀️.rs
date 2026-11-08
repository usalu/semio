//! ↩️ `change-critical-region-hooped-length-m` inverse: restores the previous `hooped_length_m`.

use crate::change_critical_region_hooped_length_m::ChangeCriticalRegionHoopedLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionHoopedLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_hooped_length_m::ChangeCriticalRegionHoopedLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionHoopedLengthM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionHoopedLengthM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionHoopedLengthM { id: payload.id.clone(), new_hooped_length_m: r0.hooped_length_m })
}
