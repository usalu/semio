use crate::change_deck_deck_width::ChangeDeckDeckWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckDeckWidth, base: &En1995Snapshot) -> Option<crate::change_deck_deck_width::ChangeDeckDeckWidth> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckDeckWidth { deck_id: payload.deck_id.clone(), new_value: item.deck_width_m }),
        None => None,
    }
}
