//! Diff for `insert-roofs`.
use super::InsertRoofs;
use crate::artifact_schema::diff::En1991RoofsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &InsertRoofs, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.roofs.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    let mut values = base.roofs.clone();
    values.insert(payload.index, payload.item.clone());
    protocol::MutationOutcome::new(En1991Diff { roofs: Some(En1991RoofsList { values }), ..Default::default() })
}
