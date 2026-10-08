//! ➖️ `remove-load-case` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveLoadCase;
use crate::diff::{En1993Diff, En1993LoadCaseDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveLoadCase, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.load_cases.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("load-case index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { load_cases: En1993LoadCaseDelta::removal(&row.id), ..Default::default() })
}
