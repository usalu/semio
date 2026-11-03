use crate::change_deck_bar_spacing::ChangeDeckBarSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckBarSpacing, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_bar_spacing::ChangeDeckBarSpacing> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckBarSpacing { deck_id: payload.deck_id.clone(), new_value: item.bar_spacing_m })],
        None => Vec::new(),
    })
}
