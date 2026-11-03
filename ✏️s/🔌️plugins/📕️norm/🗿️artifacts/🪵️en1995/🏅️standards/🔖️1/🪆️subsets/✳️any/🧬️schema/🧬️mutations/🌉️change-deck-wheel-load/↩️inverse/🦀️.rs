use crate::change_deck_wheel_load::ChangeDeckWheelLoad;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckWheelLoad, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_wheel_load::ChangeDeckWheelLoad> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckWheelLoad { deck_id: payload.deck_id.clone(), new_value: item.wheel_load_n })],
        None => Vec::new(),
    })
}
