//! ➖️ `remove-load-case` diff — removes the row at the index.

use super::RemoveLoadCase;
use crate::diff::{En1993Diff, En1993LoadCaseDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveLoadCase, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("load-case index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { load_cases: En1993LoadCaseDelta::removal(&base.load_cases, payload.index), ..Default::default() })
}
