//! ✂️ `remove-effect` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveEffect;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990EffectEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveEffect, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.effects.len() {
        return MutationOutcome::error("mutation.target-missing", "effects index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { effects: vec![En1990EffectEdit::remove(payload.index, String::new())], ..En1990Diff::default() })
}
