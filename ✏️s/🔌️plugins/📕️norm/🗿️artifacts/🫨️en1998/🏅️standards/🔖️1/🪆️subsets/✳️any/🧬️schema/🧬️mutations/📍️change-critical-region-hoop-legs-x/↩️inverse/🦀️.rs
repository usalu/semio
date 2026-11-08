//! ↩️ `change-critical-region-hoop-legs-x` inverse: restores the previous `hoop_legs_x`.

use crate::change_critical_region_hoop_legs_x::ChangeCriticalRegionHoopLegsX;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionHoopLegsX, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_hoop_legs_x::ChangeCriticalRegionHoopLegsX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionHoopLegsX, base: &En1998Snapshot) -> Option<ChangeCriticalRegionHoopLegsX> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionHoopLegsX { id: payload.id.clone(), new_hoop_legs_x: r0.hoop_legs_x })
}
