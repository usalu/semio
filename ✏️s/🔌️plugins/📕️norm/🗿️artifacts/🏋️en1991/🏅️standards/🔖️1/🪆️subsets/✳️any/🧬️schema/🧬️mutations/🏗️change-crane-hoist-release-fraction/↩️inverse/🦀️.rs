//! ↩️ `change-crane-hoist-release-fraction` inverse: restores the previous `release_fraction`.

use crate::change_crane_hoist_release_fraction::ChangeCraneHoistReleaseFraction;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneHoistReleaseFraction, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_hoist_release_fraction::ChangeCraneHoistReleaseFraction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCraneHoistReleaseFraction, base: &En1991Snapshot) -> Option<ChangeCraneHoistReleaseFraction> {
    let r0 = base.crane_hoists.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCraneHoistReleaseFraction { id: payload.id.clone(), new_release_fraction: r0.release_fraction })
}
