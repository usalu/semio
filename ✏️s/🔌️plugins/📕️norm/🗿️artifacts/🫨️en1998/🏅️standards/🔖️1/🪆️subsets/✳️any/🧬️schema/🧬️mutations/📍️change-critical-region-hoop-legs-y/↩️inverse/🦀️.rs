//! ↩️ `change-critical-region-hoop-legs-y` inverse: restores the previous `hoop_legs_y`.

use crate::change_critical_region_hoop_legs_y::ChangeCriticalRegionHoopLegsY;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionHoopLegsY, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_hoop_legs_y::ChangeCriticalRegionHoopLegsY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionHoopLegsY, base: &En1998Snapshot) -> Option<ChangeCriticalRegionHoopLegsY> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionHoopLegsY { id: payload.id.clone(), new_hoop_legs_y: r0.hoop_legs_y })
}
