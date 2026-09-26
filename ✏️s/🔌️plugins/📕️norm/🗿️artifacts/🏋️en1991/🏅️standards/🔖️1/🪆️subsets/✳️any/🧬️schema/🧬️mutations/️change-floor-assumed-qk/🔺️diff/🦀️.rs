//! Diff for `change-floor-assumed-qk`.
use super::ChangeFloorAssumedQk;
use crate::artifact_schema::diff::En1991FloorsList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.floors.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    if base.floors[payload.index].assumed_qk == payload.new_assumed_qk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    let mut values = base.floors.clone();
    values[payload.index].assumed_qk = payload.new_assumed_qk;
    protocol::MutationOutcome::new(En1991Diff { floors: Some(En1991FloorsList { values }), ..Default::default() })
}
