//! 🔺️ Diff for `insert-roofs`.
use super::InsertRoofs;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991RoofDelta};
pub fn diff(payload: &InsertRoofs, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.roofs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.roofs.iter().any(|existing| existing.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
