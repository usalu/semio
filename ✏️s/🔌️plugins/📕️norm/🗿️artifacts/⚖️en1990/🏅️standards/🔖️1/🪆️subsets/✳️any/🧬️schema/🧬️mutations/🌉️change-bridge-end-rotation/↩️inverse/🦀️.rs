//! ↩️ `change-bridge-end-rotation` inverse: restores the previous `end_rotation`.

use crate::change_bridge_end_rotation::ChangeBridgeEndRotation;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeEndRotation, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_end_rotation::ChangeBridgeEndRotation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeEndRotation, base: &En1990Snapshot) -> Option<ChangeBridgeEndRotation> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeEndRotation { id: payload.id.clone(), new_end_rotation: r0.end_rotation })
}
