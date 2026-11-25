use crate::insert_deck::InsertDeck;
use crate::remove_deck;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertDeck, base: &En1995Snapshot) -> Option<crate::remove_deck::RemoveDeck> {
    let at = payload.index.min(base.decks.len());
    Some(remove_deck::RemoveDeck { index: at })
}
