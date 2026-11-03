//! ↩️ `change-bridge-deck-accel-limit` inverse: restores the previous `deck_acceleration_limit`.

use crate::change_bridge_deck_accel_limit::ChangeBridgeDeckAccelLimit;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckAccelLimit, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_accel_limit::ChangeBridgeDeckAccelLimit> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckAccelLimit, base: &En1990Snapshot) -> Option<ChangeBridgeDeckAccelLimit> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckAccelLimit { id: payload.id.clone(), new_deck_acceleration_limit: r0.deck_acceleration_limit })
}
