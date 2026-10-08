//! 🗼 `insert-tower` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertTower;
use crate::diff::{En1998Diff, En1998TowerDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.towers.iter().any(|existing| existing.id == payload.tower.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tower id {} already exists.", payload.tower.id), [payload.tower.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.towers.len());
    protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::insertion(index, payload.tower.clone()), ..Default::default() })
}
