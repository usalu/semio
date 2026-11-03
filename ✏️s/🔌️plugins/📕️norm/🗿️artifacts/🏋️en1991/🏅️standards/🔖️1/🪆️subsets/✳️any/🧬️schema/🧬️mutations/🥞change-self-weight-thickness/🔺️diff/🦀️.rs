//! 🥞 `change-self-weight-thickness` diff: patches `thickness` of the addressed self weight element row.

use crate::change_self_weight_thickness::ChangeSelfWeightThickness;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SelfWeightElementDelta, En1991SelfWeightElementPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSelfWeightThickness, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_thickness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.self_weight_elements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("self weight element {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.thickness == payload.new_thickness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The self weight element already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: En1991SelfWeightElementDelta::modification(&payload.id, En1991SelfWeightElementPatch { thickness: Some(payload.new_thickness), ..Default::default() }), ..Default::default() })
}
