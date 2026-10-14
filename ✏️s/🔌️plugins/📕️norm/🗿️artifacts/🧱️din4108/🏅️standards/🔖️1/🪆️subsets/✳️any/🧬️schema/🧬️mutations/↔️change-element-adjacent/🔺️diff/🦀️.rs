//! ↔️ `change-element-adjacent` diff — patches the row's `adjacent`; an id the document does not hold is a `mutation.invariant`.

use crate::change_element_adjacent::ChangeElementAdjacent;
use semio_s_artifact_norm_din4108_diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch};
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &ChangeElementAdjacent, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let patch = Din4108ElementPatch { adjacent: Some(payload.new_adjacent.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&row.id, patch), ..Default::default() })
}
