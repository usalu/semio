//! 📏 `change-load-dispersal-wheel-length` diff: patches `wheel_length` of the addressed load dispersal row.

use crate::change_load_dispersal_wheel_length::ChangeLoadDispersalWheelLength;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LoadDispersalDelta, En1991LoadDispersalPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLoadDispersalWheelLength, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_wheel_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.load_dispersals.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load dispersal {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.wheel_length == payload.new_wheel_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The load dispersal already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { load_dispersals: En1991LoadDispersalDelta::modification(&payload.id, En1991LoadDispersalPatch { wheel_length: Some(payload.new_wheel_length), ..Default::default() }), ..Default::default() })
}
