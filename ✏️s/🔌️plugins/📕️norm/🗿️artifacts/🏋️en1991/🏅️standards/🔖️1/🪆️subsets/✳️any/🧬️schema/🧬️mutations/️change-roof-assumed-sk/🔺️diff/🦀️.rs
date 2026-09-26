//! Diff for `change-roof-assumed-sk`.
use super::ChangeRoofAssumedSk;
use crate::artifact_schema::diff::En1991RoofsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.roofs.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    if base.roofs[payload.index].assumed_sk == payload.new_assumed_sk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    let mut values = base.roofs.clone();
    values[payload.index].assumed_sk = payload.new_assumed_sk;
    protocol::MutationOutcome::new(En1991Diff { roofs: Some(En1991RoofsList { values }), ..Default::default() })
}
