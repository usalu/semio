//! ⚙ `change-snow-guard-slope-length` diff: patches `slope_length` of the addressed snow guard row.

use crate::change_snow_guard_slope_length::ChangeSnowGuardSlopeLength;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SnowGuardDelta, En1991SnowGuardPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSnowGuardSlopeLength, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_slope_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.snow_guards.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Snow guard {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.slope_length == payload.new_slope_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The snow guard already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { snow_guards: En1991SnowGuardDelta::modification(&payload.id, En1991SnowGuardPatch { slope_length: Some(payload.new_slope_length), ..Default::default() }), ..Default::default() })
}
