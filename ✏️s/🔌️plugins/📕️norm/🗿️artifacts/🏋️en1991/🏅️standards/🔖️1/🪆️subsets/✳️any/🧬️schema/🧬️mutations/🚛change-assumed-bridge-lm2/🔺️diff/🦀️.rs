//! 🔺️ Diff for `change-assumed-bridge-lm2`.
use crate::change_assumed_bridge_lm2::ChangeAssumedBridgeLm2;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedBridgeLm2, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_lm2 == payload.new_assumed_bridge_lm2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_lm2: Some(payload.new_assumed_bridge_lm2), ..Default::default() })
}
