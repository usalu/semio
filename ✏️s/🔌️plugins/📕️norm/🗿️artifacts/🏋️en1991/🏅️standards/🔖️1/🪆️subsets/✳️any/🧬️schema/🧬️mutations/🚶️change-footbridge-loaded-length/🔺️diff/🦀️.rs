//! 🚶 `change-footbridge-loaded-length` diff: patches `loaded_length` of the addressed footbridge row.

use crate::change_footbridge_loaded_length::ChangeFootbridgeLoadedLength;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FootbridgeDelta, En1991FootbridgePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFootbridgeLoadedLength, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_loaded_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.footbridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Footbridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.loaded_length == payload.new_loaded_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The footbridge already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { footbridges: En1991FootbridgeDelta::modification(&payload.id, En1991FootbridgePatch { loaded_length: Some(payload.new_loaded_length), ..Default::default() }), ..Default::default() })
}
