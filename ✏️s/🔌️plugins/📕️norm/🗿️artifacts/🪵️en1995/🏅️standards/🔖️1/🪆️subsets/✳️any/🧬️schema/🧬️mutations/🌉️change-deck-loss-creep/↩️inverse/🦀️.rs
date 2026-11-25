use crate::change_deck_loss_creep::ChangeDeckLossCreep;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckLossCreep, base: &En1995Snapshot) -> Option<crate::change_deck_loss_creep::ChangeDeckLossCreep> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckLossCreep { deck_id: payload.deck_id.clone(), new_value: item.loss_creep }),
        None => None,
    }
}
