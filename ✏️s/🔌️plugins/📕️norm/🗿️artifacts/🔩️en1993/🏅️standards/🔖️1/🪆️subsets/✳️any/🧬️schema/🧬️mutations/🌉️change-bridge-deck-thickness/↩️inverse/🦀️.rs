//! ↩️ `change-bridge-deck-thickness` inverse: restores the previous `deck_thickness`.

use crate::change_bridge_deck_thickness::ChangeBridgeDeckThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_thickness::ChangeBridgeDeckThickness> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeDeckThickness { id: payload.id.clone(), new_deck_thickness: row.deck_thickness })],
        None => Vec::new(),
    })
}
