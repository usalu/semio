//! 🛢️ `insert-tank` diff: inserts the tank row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_tank::InsertTank;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TankDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.tanks.iter().any(|existing| existing.id == payload.tank.id) {
        let key = payload.tank.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("tank {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.tanks.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { tanks: En1998TankDelta::insertion(index, payload.tank.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the tank list; inserted at {index}.", payload.index.unwrap_or(index)))
}
