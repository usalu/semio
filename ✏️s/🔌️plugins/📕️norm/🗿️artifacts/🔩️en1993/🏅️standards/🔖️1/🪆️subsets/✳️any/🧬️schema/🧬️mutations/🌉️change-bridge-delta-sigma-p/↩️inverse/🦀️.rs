//! ↩️ `change-bridge-delta-sigma-p` inverse: restores the previous `delta_sigma_p`.

use crate::change_bridge_delta_sigma_p::ChangeBridgeDeltaSigmaP;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeltaSigmaP, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_delta_sigma_p::ChangeBridgeDeltaSigmaP> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeDeltaSigmaP { id: payload.id.clone(), new_delta_sigma_p: row.delta_sigma_p })],
        None => Vec::new(),
    })
}
