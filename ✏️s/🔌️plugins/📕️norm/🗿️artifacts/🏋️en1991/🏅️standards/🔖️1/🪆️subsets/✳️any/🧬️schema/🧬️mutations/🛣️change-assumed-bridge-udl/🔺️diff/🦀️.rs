//! 🔺️ Diff for `change-assumed-bridge-udl`.
use crate::change_assumed_bridge_udl::ChangeAssumedBridgeUdl;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedBridgeUdl, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_udl == payload.new_assumed_bridge_udl {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_udl: Some(payload.new_assumed_bridge_udl), ..Default::default() })
}
