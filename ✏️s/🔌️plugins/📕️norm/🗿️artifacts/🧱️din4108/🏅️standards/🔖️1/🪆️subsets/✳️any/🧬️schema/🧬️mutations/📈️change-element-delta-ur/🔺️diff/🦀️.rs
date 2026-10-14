//! 📈️ `change-element-delta-ur` diff — patches the row's `delta_u_r`; an id the document does not hold is a `mutation.invariant`.

use crate::change_element_delta_u_r::ChangeElementDeltaUr;
use semio_s_artifact_norm_din4108_diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch};
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &ChangeElementDeltaUr, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let patch = Din4108ElementPatch { delta_u_r: Some(payload.new_delta_u_r), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&row.id, patch), ..Default::default() })
}
