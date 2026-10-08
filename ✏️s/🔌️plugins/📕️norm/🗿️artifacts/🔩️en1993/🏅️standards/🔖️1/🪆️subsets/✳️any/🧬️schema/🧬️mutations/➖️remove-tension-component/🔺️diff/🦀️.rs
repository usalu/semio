//! ➖️ `remove-tension-component` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveTensionComponent;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993TensionComponentEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tension_components.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tension-component index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { tension_components: vec![En1993TensionComponentEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
