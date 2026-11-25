use crate::change_deck_loss_moisture::ChangeDeckLossMoisture;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckLossMoisture, base: &En1995Snapshot) -> Option<crate::change_deck_loss_moisture::ChangeDeckLossMoisture> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckLossMoisture { deck_id: payload.deck_id.clone(), new_value: item.loss_moisture }),
        None => None,
    }
}
