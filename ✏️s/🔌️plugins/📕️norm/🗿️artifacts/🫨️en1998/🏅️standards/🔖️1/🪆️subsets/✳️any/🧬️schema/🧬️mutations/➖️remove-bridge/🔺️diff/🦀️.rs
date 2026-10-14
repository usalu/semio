//! ➖️ `remove-bridge` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_bridge::RemoveBridge;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998BridgeDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.bridges.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::removal(&base.bridges, payload.index), ..Default::default() })
}
