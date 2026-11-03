use crate::change_deck_compression_c90::ChangeDeckCompressionC90;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckCompressionC90, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_compression_c90::ChangeDeckCompressionC90> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckCompressionC90 { deck_id: payload.deck_id.clone(), new_value: item.compression_perpendicular_pa })],
        None => Vec::new(),
    })
}
