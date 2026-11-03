//! 🔺️ `change-pile-set-per-blow` diff: patches `set_per_blow` of the addressed pile row.

use crate::change_pile_set_per_blow::ChangePileSetPerBlow;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileSetPerBlow, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_set_per_blow.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.set_per_blow == payload.new_set_per_blow {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { set_per_blow: Some(payload.new_set_per_blow), ..Default::default() })), ..Default::default() })
}
