//! ↩️ `change-critical-region-hoop-fywd` inverse: restores the previous `hoop_design_yield_mpa`.

use crate::change_critical_region_hoop_fywd::ChangeCriticalRegionHoopDesignYieldMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionHoopDesignYieldMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_hoop_fywd::ChangeCriticalRegionHoopDesignYieldMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionHoopDesignYieldMpa, base: &En1998Snapshot) -> Option<ChangeCriticalRegionHoopDesignYieldMpa> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionHoopDesignYieldMpa { id: payload.id.clone(), new_hoop_design_yield_mpa: r0.hoop_design_yield_mpa })
}
