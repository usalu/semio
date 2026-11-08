//! ↩️ `change-critical-region-concrete-fcd` inverse: restores the previous `concrete_design_strength_mpa`.

use crate::change_critical_region_concrete_fcd::ChangeCriticalRegionConcreteDesignStrengthMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionConcreteDesignStrengthMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_concrete_fcd::ChangeCriticalRegionConcreteDesignStrengthMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionConcreteDesignStrengthMpa, base: &En1998Snapshot) -> Option<ChangeCriticalRegionConcreteDesignStrengthMpa> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionConcreteDesignStrengthMpa { id: payload.id.clone(), new_concrete_design_strength_mpa: r0.concrete_design_strength_mpa })
}
