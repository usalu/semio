use crate::insert_deck::InsertDeck;
use crate::remove_deck;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertDeck, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_deck::RemoveDeck> {
    let at = payload.index.min(base.decks.len());
    Ok(vec![Op::from(remove_deck::RemoveDeck { index: at })])
}
