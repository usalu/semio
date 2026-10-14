//! 🔺️ Diff for `change-assumed-bridge-tandem`.
use crate::change_assumed_bridge_tandem::ChangeAssumedBridgeTandem;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedBridgeTandem, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_bridge_tandem == payload.new_assumed_bridge_tandem {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_bridge_tandem: Some(payload.new_assumed_bridge_tandem), ..Default::default() })
}
