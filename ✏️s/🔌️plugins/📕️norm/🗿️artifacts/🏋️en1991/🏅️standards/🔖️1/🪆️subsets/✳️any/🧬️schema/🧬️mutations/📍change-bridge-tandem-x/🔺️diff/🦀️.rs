//! 🔺️ Diff for `change-bridge-tandem-x`.
use crate::change_bridge_tandem_x::ChangeBridgeTandemX;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeBridgeTandemX, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_tandem_x == payload.new_bridge_tandem_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_tandem_x: Some(payload.new_bridge_tandem_x), ..Default::default() })
}
