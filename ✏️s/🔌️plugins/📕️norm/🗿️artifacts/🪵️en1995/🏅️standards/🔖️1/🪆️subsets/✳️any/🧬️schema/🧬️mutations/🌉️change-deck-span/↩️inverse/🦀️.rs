use crate::change_deck_span::ChangeDeckSpan;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckSpan, base: &En1995Snapshot) -> Option<crate::change_deck_span::ChangeDeckSpan> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckSpan { deck_id: payload.deck_id.clone(), new_value: item.span_m }),
        None => None,
    }
}
