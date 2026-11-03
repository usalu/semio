use crate::remove_deck::RemoveDeck;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995DeckDelta;
pub fn diff(payload: &RemoveDeck, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.decks.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Deck index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { decks: En1995DeckDelta::removal(&base.decks, payload.index), ..Default::default() })
}
