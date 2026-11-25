use crate::remove_deck::RemoveDeck;
use crate::insert_deck;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveDeck, base: &En1995Snapshot) -> Option<crate::insert_deck::InsertDeck> {
    match base.decks.get(payload.index) {
        Some(item) => Some(insert_deck::InsertDeck { index: payload.index, deck: item.clone() }),
        None => None,
    }
}
