//! 🧭 `change-element-orientation-deg` diff — patches the row's `orientation_deg`; an id the document does not hold is a `mutation.invariant`.

use super::ChangeElementOrientationDeg;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ElementEdit, Din4108ElementPatch};
use crate::Din4108Snapshot;

pub fn diff(payload: &ChangeElementOrientationDeg, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some((index, row)) = base.elements.iter().enumerate().find(|(_, row)| row.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element not found", Vec::<String>::new());
    };
    let patch = Din4108ElementPatch { orientation_deg: Some(payload.new_orientation_deg), ..Default::default() };
    protocol::MutationOutcome::new(Din4108Diff { elements: vec![Din4108ElementEdit::patch(index, row.id.clone(), patch)], ..Default::default() })
}
