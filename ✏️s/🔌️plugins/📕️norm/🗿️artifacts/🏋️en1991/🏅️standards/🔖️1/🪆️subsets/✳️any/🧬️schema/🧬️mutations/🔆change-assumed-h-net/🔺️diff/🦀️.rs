//! 🔺️ Diff for `change-assumed-h-net`.
use crate::change_assumed_h_net::ChangeAssumedHNet;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedHNet, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_h_net == payload.new_assumed_h_net {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_h_net: Some(payload.new_assumed_h_net), ..Default::default() })
}
