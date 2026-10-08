//! 🔺️ Diff for `insert-floors`.
use super::InsertFloors;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991FloorDelta};
pub fn diff(payload: &InsertFloors, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.floors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.floors.iter().any(|existing| existing.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
