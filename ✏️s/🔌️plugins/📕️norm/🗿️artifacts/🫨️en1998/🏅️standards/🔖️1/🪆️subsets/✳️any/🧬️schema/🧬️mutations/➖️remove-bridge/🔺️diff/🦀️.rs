//! ➖️ `remove-bridge` diff: removes the addressed bridge row.

use crate::remove_bridge::RemoveBridge;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.bridges.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::removal(&base.bridges, index), ..Default::default() })
}
