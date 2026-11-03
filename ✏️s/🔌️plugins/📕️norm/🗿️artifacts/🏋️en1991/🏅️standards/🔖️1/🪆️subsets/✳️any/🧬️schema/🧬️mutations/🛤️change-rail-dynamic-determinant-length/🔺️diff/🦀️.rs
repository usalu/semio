//! 🛤 `change-rail-dynamic-determinant-length` diff: patches `determinant_length` of the addressed rail span row.

use crate::change_rail_dynamic_determinant_length::ChangeRailDynamicDeterminantLength;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDynamicDelta, En1991RailDynamicPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailDynamicDeterminantLength, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_determinant_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.rail_dynamics.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail span {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.determinant_length == payload.new_determinant_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The rail span already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_dynamics: En1991RailDynamicDelta::modification(&payload.id, En1991RailDynamicPatch { determinant_length: Some(payload.new_determinant_length), ..Default::default() }), ..Default::default() })
}
