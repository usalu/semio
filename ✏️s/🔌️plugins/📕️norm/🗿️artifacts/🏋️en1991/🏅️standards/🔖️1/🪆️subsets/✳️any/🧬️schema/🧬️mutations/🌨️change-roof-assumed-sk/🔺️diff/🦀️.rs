//! 🔺️ Diff for `change-roof-assumed-sk`.
use super::ChangeRoofAssumedSk;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991RoofDelta, En1991RoofPatch};
pub fn diff(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.roofs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.roofs[payload.index].assumed_sk == payload.new_assumed_sk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    let roof = &base.roofs[payload.index];
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::modification(&roof.id, En1991RoofPatch { assumed_sk: Some(payload.new_assumed_sk), ..Default::default() }), ..Default::default() })
}
