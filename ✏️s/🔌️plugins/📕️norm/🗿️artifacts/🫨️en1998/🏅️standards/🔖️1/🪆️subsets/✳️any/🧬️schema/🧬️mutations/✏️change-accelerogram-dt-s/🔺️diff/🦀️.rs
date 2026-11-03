//! ✏️ `change-accelerogram-dt-s` diff: patches `dt_s` of the addressed accelerogram row.

use crate::change_accelerogram_dt_s::ChangeAccelerogramDtS;
use semio_s_artifact_norm_en1998_diff::{En1998AccelerogramDelta, En1998AccelerogramPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAccelerogramDtS, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_dt_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.accelerograms.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("accelerogram {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.dt_s == payload.new_dt_s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accelerogram already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { accelerograms: En1998AccelerogramDelta::modification(&payload.id, En1998AccelerogramPatch { dt_s: Some(payload.new_dt_s), ..Default::default() }), ..Default::default() })
}
