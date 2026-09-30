//! Diff for `remove-foundation`.
use super::RemoveFoundation;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.foundations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.foundations.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { foundations: Some(items), ..Default::default() })
}
