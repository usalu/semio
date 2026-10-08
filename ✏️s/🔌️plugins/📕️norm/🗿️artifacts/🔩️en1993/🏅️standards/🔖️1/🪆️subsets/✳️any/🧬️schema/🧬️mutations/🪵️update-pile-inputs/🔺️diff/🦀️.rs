//! 🪵️ `update-pile-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdatePileInputs;
use crate::diff::{En1993Diff, En1993PileDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdatePileInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.piles.iter().position(|row| row.id == payload.pile.id) {
        Some(index) if base.piles[index] == payload.pile => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993PileDelta::removal(&payload.pile.id);
            replacement.absorb(En1993PileDelta::insertion(&base.piles, index, payload.pile.clone()));
            replacement
        }
        None => En1993PileDelta::insertion(&base.piles, base.piles.len(), payload.pile.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { piles: delta, ..Default::default() })
}
