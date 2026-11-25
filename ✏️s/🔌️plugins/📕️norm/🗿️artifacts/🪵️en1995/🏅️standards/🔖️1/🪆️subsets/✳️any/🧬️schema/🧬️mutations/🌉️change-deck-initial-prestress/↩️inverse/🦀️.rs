use crate::change_deck_initial_prestress::ChangeDeckInitialPrestress;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDeckInitialPrestress, base: &En1995Snapshot) -> Option<crate::change_deck_initial_prestress::ChangeDeckInitialPrestress> {
    match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => Some(ChangeDeckInitialPrestress { deck_id: payload.deck_id.clone(), new_value: item.initial_prestress_pa }),
        None => None,
    }
}
