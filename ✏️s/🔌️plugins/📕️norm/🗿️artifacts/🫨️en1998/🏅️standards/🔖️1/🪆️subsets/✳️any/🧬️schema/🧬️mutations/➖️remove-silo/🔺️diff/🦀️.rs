//! Diff for `remove-silo`.
use super::RemoveSilo;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.silos.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("silo #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.silos.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { silos: Some(items), ..Default::default() })
}
