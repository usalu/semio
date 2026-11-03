//! ↩️ `change-bearing-device` inverse: restores the previous `device`.

use crate::change_bearing_device::ChangeBearingDevice;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingDevice, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_device::ChangeBearingDevice> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingDevice, base: &En1998Snapshot) -> Option<ChangeBearingDevice> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingDevice { id: payload.id.clone(), new_device: r0.device })
}
