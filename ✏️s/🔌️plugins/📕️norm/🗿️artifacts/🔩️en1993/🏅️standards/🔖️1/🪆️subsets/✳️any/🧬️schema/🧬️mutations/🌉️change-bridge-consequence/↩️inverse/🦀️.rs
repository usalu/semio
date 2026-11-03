//! ↩️ `change-bridge-consequence` inverse: restores the previous `consequence`.

use crate::change_bridge_consequence::ChangeBridgeConsequence;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeConsequence, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_consequence::ChangeBridgeConsequence> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeConsequence { id: payload.id.clone(), new_consequence: row.consequence })],
        None => Vec::new(),
    })
}
