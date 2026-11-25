use crate::change_deck_compression_c90::ChangeDeckCompressionC90;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckCompressionC90, base: &En1995Snapshot) -> Option<crate::change_deck_compression_c90::ChangeDeckCompressionC90> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckCompressionC90 { deck_id: payload.deck_id.clone(), new_value: item.compression_perpendicular_pa }),
        None => None,
    }
}
