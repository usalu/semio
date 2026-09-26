//! Diff for `change-hoist-class`.
use super::ChangeHoistClass;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeHoistClass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.hoist_class == payload.new_hoist_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { hoist_class: Some(payload.new_hoist_class.clone()), ..Default::default() })
}
