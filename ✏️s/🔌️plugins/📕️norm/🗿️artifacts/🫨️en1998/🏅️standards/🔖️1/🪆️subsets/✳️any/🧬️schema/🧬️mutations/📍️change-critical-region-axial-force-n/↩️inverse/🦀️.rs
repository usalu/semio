//! ↩️ `change-critical-region-axial-force-n` inverse: restores the previous `axial_force_n`.

use crate::change_critical_region_axial_force_n::ChangeCriticalRegionAxialForceN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionAxialForceN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_axial_force_n::ChangeCriticalRegionAxialForceN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionAxialForceN, base: &En1998Snapshot) -> Option<ChangeCriticalRegionAxialForceN> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionAxialForceN { id: payload.id.clone(), new_axial_force_n: r0.axial_force_n })
}
