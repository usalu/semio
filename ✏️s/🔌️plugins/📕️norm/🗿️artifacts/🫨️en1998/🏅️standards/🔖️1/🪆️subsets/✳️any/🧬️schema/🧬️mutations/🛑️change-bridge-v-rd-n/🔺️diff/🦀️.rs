//! 🛑️ `change-bridge-v-rd-n` diff — patches the one field of the row at the index; a missing row is a `mutation.target-missing`.

use crate::change_bridge_v_rd_n::ChangeBridgeVRdN;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998BridgeDelta, En1998BridgePatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgeVRdN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.bridges.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "bridge", Vec::<String>::new());
    };
    let patch = En1998BridgePatch { v_rd_n: Some(payload.new_v_rd_n), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&row.id, patch), ..Default::default() })
}
