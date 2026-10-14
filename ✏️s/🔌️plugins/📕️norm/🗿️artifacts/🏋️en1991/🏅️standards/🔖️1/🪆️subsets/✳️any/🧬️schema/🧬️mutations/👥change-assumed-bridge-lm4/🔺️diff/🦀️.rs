//! 🔺️ Diff for `change-assumed-bridge-lm4`.
use crate::change_assumed_bridge_lm4::ChangeAssumedBridgeLm4;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedBridgeLm4, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_lm4 == payload.new_assumed_bridge_lm4 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_lm4: Some(payload.new_assumed_bridge_lm4), ..Default::default() })
}
