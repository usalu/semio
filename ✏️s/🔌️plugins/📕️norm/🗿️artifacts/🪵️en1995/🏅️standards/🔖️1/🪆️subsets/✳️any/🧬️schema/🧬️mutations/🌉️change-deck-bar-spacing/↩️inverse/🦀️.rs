use crate::change_deck_bar_spacing::ChangeDeckBarSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckBarSpacing, base: &En1995Snapshot) -> Option<crate::change_deck_bar_spacing::ChangeDeckBarSpacing> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckBarSpacing { deck_id: payload.deck_id.clone(), new_value: item.bar_spacing_m }),
        None => None,
    }
}
