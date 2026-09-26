//! Diff for `change-assumed-construction-qk`.
use super::ChangeAssumedConstructionQk;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedConstructionQk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_construction_qk == payload.new_assumed_construction_qk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_construction_qk: Some(payload.new_assumed_construction_qk), ..Default::default() })
}
