use crate::change_deck_wheel_load::ChangeDeckWheelLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckWheelLoad, base: &En1995Snapshot) -> Option<crate::change_deck_wheel_load::ChangeDeckWheelLoad> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckWheelLoad { deck_id: payload.deck_id.clone(), new_value: item.wheel_load_n }),
        None => None,
    }
}
