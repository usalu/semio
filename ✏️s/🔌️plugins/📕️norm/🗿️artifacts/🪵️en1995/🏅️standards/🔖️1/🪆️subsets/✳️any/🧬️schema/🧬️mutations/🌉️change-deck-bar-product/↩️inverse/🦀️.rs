use crate::change_deck_bar_product::ChangeDeckBarProduct;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckBarProduct, base: &En1995Snapshot) -> Option<crate::change_deck_bar_product::ChangeDeckBarProduct> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckBarProduct { deck_id: payload.deck_id.clone(), new_value: item.bar_product }),
        None => None,
    }
}
