use crate::change_deck_deck_depth::ChangeDeckDeckDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckDeckDepth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_deck_depth::ChangeDeckDeckDepth> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckDeckDepth { deck_id: payload.deck_id.clone(), new_value: item.deck_depth_m })],
        None => Vec::new(),
    })
}
