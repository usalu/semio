//! Diff for `insert-tower`.
use super::InsertTower;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.towers.iter().any(|existing| existing.id == payload.tower.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tower id {} already exists.", payload.tower.id), [payload.tower.id.clone()]);
    }
    let mut items = base.towers.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.tower.clone());
    protocol::MutationOutcome::new(En1998Diff { towers: Some(items), ..Default::default() })
}
