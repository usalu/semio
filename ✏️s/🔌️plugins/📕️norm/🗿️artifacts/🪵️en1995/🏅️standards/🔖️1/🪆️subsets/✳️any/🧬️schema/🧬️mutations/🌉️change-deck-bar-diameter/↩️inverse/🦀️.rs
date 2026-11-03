use crate::change_deck_bar_diameter::ChangeDeckBarDiameter;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckBarDiameter, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_bar_diameter::ChangeDeckBarDiameter> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckBarDiameter { deck_id: payload.deck_id.clone(), new_value: item.bar_diameter_m })],
        None => Vec::new(),
    })
}
