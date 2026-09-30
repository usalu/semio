//! Diff for `remove-bridge`.
use super::RemoveBridge;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.bridges.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.bridges.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { bridges: Some(items), ..Default::default() })
}
