//! ➕️ `insert-tension-component` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertTensionComponent;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993TensionComponentEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.tension_components.iter().any(|existing| existing.id == payload.tension_component.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tension component id {} already exists.", payload.tension_component.id), [payload.tension_component.id.clone()]);
    }
    let index = payload.index.min(base.tension_components.len());
    protocol::MutationOutcome::new(En1993Diff { tension_components: vec![En1993TensionComponentEdit::insert(index, payload.tension_component.clone())], ..Default::default() })
}
