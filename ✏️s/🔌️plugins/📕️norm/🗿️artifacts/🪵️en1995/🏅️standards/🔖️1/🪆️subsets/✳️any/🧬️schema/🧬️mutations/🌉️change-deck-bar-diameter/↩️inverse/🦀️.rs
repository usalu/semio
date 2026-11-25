use crate::change_deck_bar_diameter::ChangeDeckBarDiameter;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckBarDiameter, base: &En1995Snapshot) -> Option<crate::change_deck_bar_diameter::ChangeDeckBarDiameter> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckBarDiameter { deck_id: payload.deck_id.clone(), new_value: item.bar_diameter_m }),
        None => None,
    }
}
