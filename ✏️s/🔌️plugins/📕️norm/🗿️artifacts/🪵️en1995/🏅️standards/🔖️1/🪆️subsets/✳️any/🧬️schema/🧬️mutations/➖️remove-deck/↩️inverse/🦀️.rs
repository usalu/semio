use crate::remove_deck::RemoveDeck;
use crate::insert_deck;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveDeck, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_deck::InsertDeck> {
    Ok(match base.decks.get(payload.index) {
        Some(item) => vec![Op::from(insert_deck::InsertDeck { index: payload.index, deck: item.clone() })],
        None => Vec::new(),
    })
}
