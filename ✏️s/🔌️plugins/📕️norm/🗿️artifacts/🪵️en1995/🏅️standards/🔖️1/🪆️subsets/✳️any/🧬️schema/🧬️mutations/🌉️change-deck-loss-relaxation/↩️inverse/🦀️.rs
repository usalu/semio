use crate::change_deck_loss_relaxation::ChangeDeckLossRelaxation;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckLossRelaxation, base: &En1995Snapshot) -> Option<crate::change_deck_loss_relaxation::ChangeDeckLossRelaxation> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckLossRelaxation { deck_id: payload.deck_id.clone(), new_value: item.loss_relaxation }),
        None => None,
    }
}
