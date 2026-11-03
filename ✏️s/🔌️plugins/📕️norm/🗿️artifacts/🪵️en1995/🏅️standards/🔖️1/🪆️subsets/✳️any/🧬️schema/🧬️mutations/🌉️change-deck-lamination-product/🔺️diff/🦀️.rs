use crate::change_deck_lamination_product::ChangeDeckLaminationProduct;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995DeckDelta, En1995DeckPatch};
pub fn diff(payload: &ChangeDeckLaminationProduct, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.decks.iter().find(|item| item.id == payload.deck_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown deck id.", vec![payload.deck_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { decks: En1995DeckDelta::modification(&row.id, En1995DeckPatch { lamination_product: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
