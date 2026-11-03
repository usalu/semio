//! 🔺️ `change-footing-length` diff: patches `length` of the addressed footing row.

use crate::change_footing_length::ChangeFootingLength;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsPatch, En1997FootingsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeFootingLength, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.footings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.length == payload.new_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The footing already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::modification(&payload.id, En1997FootingsPatch { length: Some(payload.new_length), ..Default::default() })), ..Default::default() })
}
