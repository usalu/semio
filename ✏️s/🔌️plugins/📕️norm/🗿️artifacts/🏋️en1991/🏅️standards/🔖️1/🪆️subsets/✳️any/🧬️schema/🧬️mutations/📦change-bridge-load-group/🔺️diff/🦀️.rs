//! Diff for `change-bridge-load-group`.
use super::ChangeBridgeLoadGroup;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeBridgeLoadGroup, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_load_group == payload.new_bridge_load_group {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_load_group: Some(payload.new_bridge_load_group.clone()), ..Default::default() })
}
