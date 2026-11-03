//! ↩️ `change-bridge-reference-weight` inverse: restores the previous `reference_weight`.

use crate::change_bridge_reference_weight::ChangeBridgeReferenceWeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeReferenceWeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_reference_weight::ChangeBridgeReferenceWeight> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeReferenceWeight { id: payload.id.clone(), new_reference_weight: row.reference_weight })],
        None => Vec::new(),
    })
}
