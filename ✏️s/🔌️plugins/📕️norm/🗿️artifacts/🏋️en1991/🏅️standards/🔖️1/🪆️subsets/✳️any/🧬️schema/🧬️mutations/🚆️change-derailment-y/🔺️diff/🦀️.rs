//! 🚆 `change-derailment-y` diff: patches `y` of the addressed derailment case row.

use crate::change_derailment_y::ChangeDerailmentY;
use semio_s_artifact_norm_en1991_diff::{En1991DerailmentCaseDelta, En1991DerailmentCasePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeDerailmentY, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.derailment_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Derailment case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The derailment case already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { derailment_cases: En1991DerailmentCaseDelta::modification(&payload.id, En1991DerailmentCasePatch { y: Some(payload.new_y), ..Default::default() }), ..Default::default() })
}
