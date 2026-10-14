//! ➖️ `remove-pile` diff — removes the row at the index.

use crate::remove_pile::RemovePile;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PileDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemovePile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.piles.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("pile index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::removal(&base.piles, payload.index), ..Default::default() })
}
