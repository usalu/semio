//! ↩️ `change-bridge-damping-ratio` inverse: restores the previous `damping_ratio`.

use crate::change_bridge_damping_ratio::ChangeBridgeDampingRatio;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDampingRatio, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_damping_ratio::ChangeBridgeDampingRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDampingRatio, base: &En1990Snapshot) -> Option<ChangeBridgeDampingRatio> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDampingRatio { id: payload.id.clone(), new_damping_ratio: r0.damping_ratio })
}
