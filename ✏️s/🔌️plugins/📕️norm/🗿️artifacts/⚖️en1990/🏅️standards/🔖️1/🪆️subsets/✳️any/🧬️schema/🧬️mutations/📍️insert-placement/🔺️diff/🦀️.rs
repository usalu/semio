//! 📍 `insert-placement` diff: inserts the placement row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990PlacementDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertPlacement, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.placements.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { placements: En1990PlacementDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the placement list; inserted at {index}.", payload.index.unwrap_or(index)))
}
