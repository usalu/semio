//! ↩️ `change-crane-hoist-release-device` inverse: restores the previous `release_device`.

use crate::change_crane_hoist_release_device::ChangeCraneHoistReleaseDevice;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneHoistReleaseDevice, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_hoist_release_device::ChangeCraneHoistReleaseDevice> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCraneHoistReleaseDevice, base: &En1991Snapshot) -> Option<ChangeCraneHoistReleaseDevice> {
    let r0 = base.crane_hoists.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCraneHoistReleaseDevice { id: payload.id.clone(), new_release_device: r0.release_device.clone() })
}
