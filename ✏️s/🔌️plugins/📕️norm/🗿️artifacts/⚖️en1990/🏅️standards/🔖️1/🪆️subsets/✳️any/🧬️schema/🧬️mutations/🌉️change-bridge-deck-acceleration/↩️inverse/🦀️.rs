//! ↩️ `change-bridge-deck-acceleration` inverse: restores the previous `deck_acceleration`.

use crate::change_bridge_deck_acceleration::ChangeBridgeDeckAcceleration;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckAcceleration, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_acceleration::ChangeBridgeDeckAcceleration> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckAcceleration, base: &En1990Snapshot) -> Option<ChangeBridgeDeckAcceleration> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckAcceleration { id: payload.id.clone(), new_deck_acceleration: r0.deck_acceleration })
}
