use crate::change_deck_lamination_product::ChangeDeckLaminationProduct;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckLaminationProduct, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_lamination_product::ChangeDeckLaminationProduct> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckLaminationProduct { deck_id: payload.deck_id.clone(), new_value: item.lamination_product })],
        None => Vec::new(),
    })
}
