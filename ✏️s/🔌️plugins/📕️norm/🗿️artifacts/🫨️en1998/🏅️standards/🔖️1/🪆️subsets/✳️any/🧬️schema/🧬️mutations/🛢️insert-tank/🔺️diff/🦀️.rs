//! 🛢️ `insert-tank` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertTank;
use crate::diff::{En1998Diff, En1998TankDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.tanks.iter().any(|existing| existing.id == payload.tank.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tank id {} already exists.", payload.tank.id), [payload.tank.id.clone()]);
    }
    let index = payload.index.min(base.tanks.len());
    protocol::MutationOutcome::new(En1998Diff { tanks: En1998TankDelta::insertion(&base.tanks, index, payload.tank.clone()), ..Default::default() })
}
