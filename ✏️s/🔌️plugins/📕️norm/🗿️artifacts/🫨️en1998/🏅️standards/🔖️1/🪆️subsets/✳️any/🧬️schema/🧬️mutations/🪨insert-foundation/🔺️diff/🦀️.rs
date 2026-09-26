//! Diff for `insert-foundation`.
use super::InsertFoundation;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut items = base.foundations.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.foundation.clone());
    protocol::MutationOutcome::new(En1998Diff { foundations: Some(items), ..Default::default() })
}
