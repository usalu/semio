use crate::change_deck_deck_depth::ChangeDeckDeckDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckDeckDepth, base: &En1995Snapshot) -> Option<crate::change_deck_deck_depth::ChangeDeckDeckDepth> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckDeckDepth { deck_id: payload.deck_id.clone(), new_value: item.deck_depth_m }),
        None => None,
    }
}
