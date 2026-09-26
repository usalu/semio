//! Diff for `change-bridge-span`.
use super::ChangeBridgeSpan;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeBridgeSpan, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_span == payload.new_bridge_span {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_span: Some(payload.new_bridge_span), ..Default::default() })
}
