//! Diff for `insert-tower`.
use super::InsertTower;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.towers.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.tower.clone());
    protocol::MutationOutcome::new(En1998Diff { towers: Some(items), ..Default::default() })
}
