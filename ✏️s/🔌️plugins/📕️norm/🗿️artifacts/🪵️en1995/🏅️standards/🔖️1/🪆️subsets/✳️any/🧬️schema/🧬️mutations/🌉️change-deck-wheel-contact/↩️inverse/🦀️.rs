use crate::change_deck_wheel_contact::ChangeDeckWheelContact;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckWheelContact, base: &En1995Snapshot) -> Option<crate::change_deck_wheel_contact::ChangeDeckWheelContact> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckWheelContact { deck_id: payload.deck_id.clone(), new_value: item.wheel_contact_m }),
        None => None,
    }
}
