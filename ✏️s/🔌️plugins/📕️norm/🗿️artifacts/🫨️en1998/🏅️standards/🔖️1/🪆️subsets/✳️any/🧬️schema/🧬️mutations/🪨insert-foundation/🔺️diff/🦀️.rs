//! Diff for `insert-foundation`.
use super::InsertFoundation;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &InsertFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.foundations.iter().any(|existing| existing.id == payload.foundation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Foundation id {} already exists.", payload.foundation.id), [payload.foundation.id.clone()]);
    }
    let mut items = base.foundations.clone();
    let index = payload.index.min(items.len());
    items.insert(index, payload.foundation.clone());
    protocol::MutationOutcome::new(En1998Diff { foundations: Some(items), ..Default::default() })
}
