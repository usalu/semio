use crate::change_deck_wheel_contact::ChangeDeckWheelContact;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckWheelContact, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_wheel_contact::ChangeDeckWheelContact> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckWheelContact { deck_id: payload.deck_id.clone(), new_value: item.wheel_contact_m })],
        None => Vec::new(),
    })
}
