//! ➖️ `remove-tension-component` diff — removes the row at the index.

use super::RemoveTensionComponent;
use crate::diff::{En1993Diff, En1993TensionComponentDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.tension_components.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tension-component index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::removal(&base.tension_components, payload.index), ..Default::default() })
}
