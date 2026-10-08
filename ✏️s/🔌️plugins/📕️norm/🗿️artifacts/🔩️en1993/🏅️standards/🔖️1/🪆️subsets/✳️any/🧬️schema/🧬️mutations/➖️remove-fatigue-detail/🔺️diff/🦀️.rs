//! ➖️ `remove-fatigue-detail` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveFatigueDetail;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993FatigueDetailEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_details.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fatigue-detail index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: vec![En1993FatigueDetailEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
