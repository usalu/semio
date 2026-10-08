//! 🪨 `insert-foundation` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use super::InsertFoundation;
use crate::diff::En1998RowEdit as _;
use crate::diff::{En1998Diff, En1998FoundationEdit};
use crate::En1998Snapshot;

pub fn diff(payload: &InsertFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.foundations.iter().any(|existing| existing.id == payload.foundation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Foundation id {} already exists.", payload.foundation.id), [payload.foundation.id.clone()]);
    }
    let index = payload.index.min(base.foundations.len());
    protocol::MutationOutcome::new(En1998Diff { foundations: vec![En1998FoundationEdit::insert(index, payload.foundation.clone())], ..Default::default() })
}
