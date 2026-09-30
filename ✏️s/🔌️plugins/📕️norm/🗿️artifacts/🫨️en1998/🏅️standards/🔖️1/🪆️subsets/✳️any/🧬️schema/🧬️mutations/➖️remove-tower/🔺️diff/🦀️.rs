//! Diff for `remove-tower`.
use super::RemoveTower;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.towers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tower #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.towers.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { towers: Some(items), ..Default::default() })
}
