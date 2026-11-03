use crate::change_deck_lamination_width::ChangeDeckLaminationWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckLaminationWidth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_lamination_width::ChangeDeckLaminationWidth> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckLaminationWidth { deck_id: payload.deck_id.clone(), new_value: item.lamination_width_m })],
        None => Vec::new(),
    })
}
