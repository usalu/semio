//! 🗼 `insert-tower` diff: inserts the tower row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_tower::InsertTower;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TowerDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertTower, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.towers.iter().any(|existing| existing.id == payload.tower.id) {
        let key = payload.tower.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("tower {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.towers.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { towers: En1998TowerDelta::insertion(index, payload.tower.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the tower list; inserted at {index}.", payload.index.unwrap_or(index)))
}
