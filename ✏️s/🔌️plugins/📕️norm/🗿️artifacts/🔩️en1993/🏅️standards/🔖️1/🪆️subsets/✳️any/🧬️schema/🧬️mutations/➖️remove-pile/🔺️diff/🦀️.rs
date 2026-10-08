//! ➖️ `remove-pile` diff — removes the row at the index, guarded by the row's own id.

use super::RemovePile;
use crate::diff::{En1993Diff, En1993PileDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemovePile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.piles.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("pile index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::removal(&row.id), ..Default::default() })
}
