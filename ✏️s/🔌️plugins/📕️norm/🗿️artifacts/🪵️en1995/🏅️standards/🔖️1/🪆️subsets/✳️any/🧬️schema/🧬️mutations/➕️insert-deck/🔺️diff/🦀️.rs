use crate::insert_deck::InsertDeck;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995DeckDelta;
pub fn diff(payload: &InsertDeck, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.decks.iter().any(|existing| existing.id == payload.deck.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Deck id {} already exists.", payload.deck.id), [payload.deck.id.clone()]);
    }
    let at = payload.index.min(base.decks.len());
    protocol::MutationOutcome::new(En1995Diff { decks: En1995DeckDelta::insertion(at, payload.deck.clone()), ..Default::default() })
}
