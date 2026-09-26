//! Diff for `change-crane-class`.
use super::ChangeCraneClass;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeCraneClass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_class == payload.new_crane_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_class: Some(payload.new_crane_class.clone()), ..Default::default() })
}
