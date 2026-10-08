//! 📎 `insert-effect` diff — inserts the row at its position; a position past the collection's end inserts it last as a
//! `mutation.clamped` warning.

use super::InsertEffect;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990EffectEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &InsertEffect, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    let index = payload.index.unwrap_or(usize::MAX).min(base.effects.len());
    let outcome = MutationOutcome::new(En1990Diff { effects: vec![En1990EffectEdit::insert(index, payload.item.clone())], ..En1990Diff::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the member effect list; inserted at {index}.", payload.index.unwrap_or(index)))
}
