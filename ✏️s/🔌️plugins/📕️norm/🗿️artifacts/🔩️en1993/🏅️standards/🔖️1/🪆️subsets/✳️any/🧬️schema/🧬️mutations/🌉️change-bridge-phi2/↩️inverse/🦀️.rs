//! ↩️ `change-bridge-phi2` inverse: restores the previous `phi2`.

use crate::change_bridge_phi2::ChangeBridgePhi2;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePhi2, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_phi2::ChangeBridgePhi2> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgePhi2 { id: payload.id.clone(), new_phi2: row.phi2 })],
        None => Vec::new(),
    })
}
