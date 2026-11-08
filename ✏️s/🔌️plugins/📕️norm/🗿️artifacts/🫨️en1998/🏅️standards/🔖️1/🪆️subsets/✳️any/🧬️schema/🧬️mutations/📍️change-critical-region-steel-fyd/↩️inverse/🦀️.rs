//! ↩️ `change-critical-region-steel-fyd` inverse: restores the previous `steel_design_yield_mpa`.

use crate::change_critical_region_steel_fyd::ChangeCriticalRegionSteelDesignYieldMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionSteelDesignYieldMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_steel_fyd::ChangeCriticalRegionSteelDesignYieldMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionSteelDesignYieldMpa, base: &En1998Snapshot) -> Option<ChangeCriticalRegionSteelDesignYieldMpa> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionSteelDesignYieldMpa { id: payload.id.clone(), new_steel_design_yield_mpa: r0.steel_design_yield_mpa })
}
