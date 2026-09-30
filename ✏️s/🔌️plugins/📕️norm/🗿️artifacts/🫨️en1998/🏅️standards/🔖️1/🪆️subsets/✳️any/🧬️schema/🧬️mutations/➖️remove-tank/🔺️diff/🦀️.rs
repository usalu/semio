//! Diff for `remove-tank`.
use super::RemoveTank;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &RemoveTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.tanks.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tank #{}", payload.index), [payload.index.to_string()]);
    }
    let mut items = base.tanks.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff { tanks: Some(items), ..Default::default() })
}
