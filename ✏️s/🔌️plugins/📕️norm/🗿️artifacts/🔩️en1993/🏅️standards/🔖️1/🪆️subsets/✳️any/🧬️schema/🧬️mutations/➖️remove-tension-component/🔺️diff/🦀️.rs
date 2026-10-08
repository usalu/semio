//! ➖️ `remove-tension-component` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveTensionComponent;
use crate::diff::{En1993Diff, En1993TensionComponentDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tension_components.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tension-component index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::removal(&row.id), ..Default::default() })
}
