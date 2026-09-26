//! Diff for `remove-roofs`.
use super::RemoveRoofs;
use crate::artifact_schema::diff::En1991RoofsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &RemoveRoofs, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.roofs.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    let mut values = base.roofs.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1991Diff { roofs: Some(En1991RoofsList { values }), ..Default::default() })
}
