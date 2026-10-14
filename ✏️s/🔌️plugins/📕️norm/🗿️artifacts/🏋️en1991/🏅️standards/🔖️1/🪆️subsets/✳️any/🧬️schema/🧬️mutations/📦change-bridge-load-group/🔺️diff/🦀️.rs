//! 🔺️ Diff for `change-bridge-load-group`.
use crate::change_bridge_load_group::ChangeBridgeLoadGroup;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeBridgeLoadGroup, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_load_group == payload.new_bridge_load_group {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_load_group: Some(payload.new_bridge_load_group.clone()), ..Default::default() })
}
