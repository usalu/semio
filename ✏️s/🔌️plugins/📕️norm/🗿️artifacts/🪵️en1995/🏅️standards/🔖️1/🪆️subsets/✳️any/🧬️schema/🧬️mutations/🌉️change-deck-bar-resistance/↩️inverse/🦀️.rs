use crate::change_deck_bar_resistance::ChangeDeckBarResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckBarResistance, base: &En1995Snapshot) -> Option<crate::change_deck_bar_resistance::ChangeDeckBarResistance> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckBarResistance { deck_id: payload.deck_id.clone(), new_value: item.bar_resistance_pa }),
        None => None,
    }
}
