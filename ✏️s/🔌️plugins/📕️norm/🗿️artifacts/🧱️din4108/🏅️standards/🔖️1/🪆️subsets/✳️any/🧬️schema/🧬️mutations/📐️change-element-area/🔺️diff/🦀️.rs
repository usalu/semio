//! 📐️ `change-element-area` diff — patches the row's `area_m2`; an id the document does not hold is a `mutation.invariant`.

use super::ChangeElementArea;
use crate::diff::{Din4108Diff, Din4108ElementDelta, Din4108ElementPatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeElementArea, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let patch = Din4108ElementPatch { area_m2: Some(payload.new_area_m2), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::modification(&row.id, patch), ..Default::default() })
}
