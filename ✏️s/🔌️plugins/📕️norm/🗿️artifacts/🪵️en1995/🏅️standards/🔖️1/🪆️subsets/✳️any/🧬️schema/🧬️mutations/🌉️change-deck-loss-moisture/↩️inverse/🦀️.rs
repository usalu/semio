use crate::change_deck_loss_moisture::ChangeDeckLossMoisture;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckLossMoisture, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_loss_moisture::ChangeDeckLossMoisture> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckLossMoisture { deck_id: payload.deck_id.clone(), new_value: item.loss_moisture })],
        None => Vec::new(),
    })
}
