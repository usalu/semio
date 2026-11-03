//! 🌉 `remove-bridge-sls` diff: removes the addressed bridge serviceability row.

use crate::remove_bridge_sls::RemoveBridgeSls;
use semio_s_artifact_norm_en1990_diff::{En1990BridgeSlsDelta, En1990Diff};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemoveBridgeSls, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.bridge_sls.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "bridge_sls index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { bridge_sls: En1990BridgeSlsDelta::removal(&base.bridge_sls, index), ..Default::default() })
}
