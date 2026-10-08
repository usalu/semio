//! 🔺️ `add-selection-constraint` — sparse diff construction; an out-of-range explicit index clamps to the end with `mutation.clamped`.

use super::mutation::AddSelectionConstraint;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757SelectionConstraintsRows, Iso16757SelectionConstraintsInserted};

//#region 🔖️Diff

pub fn diff(payload: &AddSelectionConstraint, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.constraints.contains(&payload.constraint) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Selection constraint on \"{}\" already exists.", payload.constraint.property_id));
    }
    let len = base.selection.constraints.len();
    let clamped = matches!(payload.index, Some(index) if index > len);
    let at = payload.index.filter(|index| *index <= len).unwrap_or(len);
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { selection_constraints: Some(Iso16757SelectionConstraintsRows { inserted: vec![Iso16757SelectionConstraintsInserted { index: at, row: payload.constraint.clone() }], ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended selection constraint on \"{}\" at the end instead.", payload.constraint.property_id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
