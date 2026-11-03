//! ✏️ `change-bearing-min-support-m` diff: patches `min_support_length_m` of the addressed bearing row.

use crate::change_bearing_min_support_m::ChangeBearingMinSupportM;
use semio_s_artifact_norm_en1998_diff::{En1998BearingDelta, En1998BearingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBearingMinSupportM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_min_support_length_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bearings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bearing {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.min_support_length_m == payload.new_min_support_length_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bearing already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bearings: En1998BearingDelta::modification(&payload.id, En1998BearingPatch { min_support_length_m: Some(payload.new_min_support_length_m), ..Default::default() }), ..Default::default() })
}
