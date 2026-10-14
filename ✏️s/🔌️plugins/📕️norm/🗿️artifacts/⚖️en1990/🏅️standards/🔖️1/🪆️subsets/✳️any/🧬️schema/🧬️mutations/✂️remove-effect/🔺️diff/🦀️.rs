//! ✂️ `remove-effect` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_effect::RemoveEffect;
use semio_s_artifact_norm_en1990_diff::En1990RowEdit as _;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990EffectEdit};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveEffect, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.effects.len() {
        return MutationOutcome::error("mutation.target-missing", "effects index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { effects: vec![En1990EffectEdit::remove(payload.index, String::new())], ..En1990Diff::default() })
}
