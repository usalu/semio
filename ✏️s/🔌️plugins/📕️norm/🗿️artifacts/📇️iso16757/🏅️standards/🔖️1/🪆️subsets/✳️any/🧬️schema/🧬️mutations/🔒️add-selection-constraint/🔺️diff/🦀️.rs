//! 🔺️ `add-selection-constraint` — sparse diff construction; an explicit index past the end is `mutation.target-missing`.

use super::mutation::AddSelectionConstraint;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757SelectionConstraintsRows, Iso16757SelectionConstraintsInserted};

//#region 🔖️Diff

pub fn diff(payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.constraints.contains(&payload.constraint) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Selection constraint on \"{}\" already exists.", payload.constraint.property_id));
    }
    let len = base.selection.constraints.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({} rows) for \"{}\".", len, payload.constraint.property_id), Vec::<String>::new());
    }
    let at = payload.index.unwrap_or(len);
    protocol::MutationOutcome::new(Iso16757Diff { selection_constraints: Some(Iso16757SelectionConstraintsRows { inserted: vec![Iso16757SelectionConstraintsInserted { index: at, row: payload.constraint.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
