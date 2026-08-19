//! 🔺️ `change-bridge-span-m` — sparse diff construction.

use super::mutation::ChangeBridgeSpanM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeBridgeSpanM, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_bridge_span_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bridge span m must be a finite number.", Vec::<String>::new());
    }
    if base.bridge_span_m == payload.new_bridge_span_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bridge span m already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_span_m: Some(payload.new_bridge_span_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
