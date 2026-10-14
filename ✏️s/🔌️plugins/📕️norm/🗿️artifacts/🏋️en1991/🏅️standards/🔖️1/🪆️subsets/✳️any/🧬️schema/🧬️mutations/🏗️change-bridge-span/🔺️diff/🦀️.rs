//! 🔺️ Diff for `change-bridge-span`.
use crate::change_bridge_span::ChangeBridgeSpan;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeBridgeSpan, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_span == payload.new_bridge_span {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_span: Some(payload.new_bridge_span), ..Default::default() })
}
