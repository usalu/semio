//! Diff for `insert-tank`.
use super::InsertTank;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.tanks.iter().any(|existing| existing.id == payload.tank.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tank id {} already exists.", payload.tank.id), [payload.tank.id.clone()]);
    }
    let mut items = base.tanks.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.tank.clone());
    protocol::MutationOutcome::new(En1998Diff { tanks: Some(items), ..Default::default() })
}
