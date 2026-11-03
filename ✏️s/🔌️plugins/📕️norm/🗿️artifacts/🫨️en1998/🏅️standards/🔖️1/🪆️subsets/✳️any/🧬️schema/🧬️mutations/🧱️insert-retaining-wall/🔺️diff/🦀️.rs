//! 🧱️ `insert-retaining-wall` diff: inserts the retaining wall row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_retaining_wall::InsertRetainingWall;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998RetainingWallDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertRetainingWall, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.retaining_walls.iter().any(|existing| existing.id == payload.wall.id) {
        let key = payload.wall.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("retaining wall {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.retaining_walls.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::insertion(index, payload.wall.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the retaining wall list; inserted at {index}.", payload.index.unwrap_or(index)))
}
