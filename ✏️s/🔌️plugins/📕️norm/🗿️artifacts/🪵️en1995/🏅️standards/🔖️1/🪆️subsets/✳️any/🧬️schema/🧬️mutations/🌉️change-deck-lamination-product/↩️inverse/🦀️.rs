use crate::change_deck_lamination_product::ChangeDeckLaminationProduct;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckLaminationProduct, base: &En1995Snapshot) -> Option<crate::change_deck_lamination_product::ChangeDeckLaminationProduct> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckLaminationProduct { deck_id: payload.deck_id.clone(), new_value: item.lamination_product }),
        None => None,
    }
}
