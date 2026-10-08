//! ➖️ `remove-pile` diff — removes the row at the index, guarded by the row's own id.

use super::RemovePile;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993PileEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemovePile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.piles.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("pile index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { piles: vec![En1993PileEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
