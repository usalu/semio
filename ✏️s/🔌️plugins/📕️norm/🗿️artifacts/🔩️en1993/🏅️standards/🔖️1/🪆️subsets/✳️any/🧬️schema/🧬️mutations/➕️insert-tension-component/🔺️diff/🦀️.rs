//! ➕️ `insert-tension-component` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertTensionComponent;
use crate::diff::{En1993Diff, En1993TensionComponentDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.tension_components.iter().any(|existing| existing.id == payload.tension_component.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tension component id {} already exists.", payload.tension_component.id), [payload.tension_component.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.tension_components.len());
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::insertion(index, payload.tension_component.clone()), ..Default::default() })
}
