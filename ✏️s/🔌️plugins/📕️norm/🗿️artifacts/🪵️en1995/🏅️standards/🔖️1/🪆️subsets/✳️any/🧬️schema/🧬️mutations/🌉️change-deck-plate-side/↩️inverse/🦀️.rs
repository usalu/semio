use crate::change_deck_plate_side::ChangeDeckPlateSide;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckPlateSide, base: &En1995Snapshot) -> Option<crate::change_deck_plate_side::ChangeDeckPlateSide> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckPlateSide { deck_id: payload.deck_id.clone(), new_value: item.plate_side_m }),
        None => None,
    }
}
