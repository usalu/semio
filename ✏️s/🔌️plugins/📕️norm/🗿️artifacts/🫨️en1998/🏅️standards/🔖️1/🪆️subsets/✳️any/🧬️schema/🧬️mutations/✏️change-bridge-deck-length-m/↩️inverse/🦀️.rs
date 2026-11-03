//! ↩️ `change-bridge-deck-length-m` inverse: restores the previous `deck_length_m`.

use crate::change_bridge_deck_length_m::ChangeBridgeDeckLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_length_m::ChangeBridgeDeckLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckLengthM, base: &En1998Snapshot) -> Option<ChangeBridgeDeckLengthM> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckLengthM { id: payload.id.clone(), new_deck_length_m: r0.deck_length_m })
}
