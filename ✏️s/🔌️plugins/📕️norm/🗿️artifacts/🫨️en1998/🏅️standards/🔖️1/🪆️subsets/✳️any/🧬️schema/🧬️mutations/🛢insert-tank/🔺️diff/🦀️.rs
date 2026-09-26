//! Diff for `insert-tank`.
use super::InsertTank;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.tanks.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.tank.clone());
    protocol::MutationOutcome::new(En1998Diff { tanks: Some(items), ..Default::default() })
}
