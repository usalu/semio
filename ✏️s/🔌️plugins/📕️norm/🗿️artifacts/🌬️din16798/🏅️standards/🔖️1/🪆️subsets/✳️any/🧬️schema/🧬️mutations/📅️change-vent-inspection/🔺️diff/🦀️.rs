//! 📅️ `change-vent-inspection` diff — patches the one field of the row with that id; an id the document does not hold is a `mutation.invariant`.

use super::ChangeVentInspection;
use crate::diff::Din16798RowEdit as _;
use crate::diff::{Din16798Diff, Din16798VentSystemEdit, Din16798VentSystemPatch};
use crate::Din16798Snapshot;

pub fn diff(payload: &ChangeVentInspection, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some((index, row)) = base.vent_systems.iter().enumerate().find(|(_, row)| row.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    let patch = Din16798VentSystemPatch { years_since_inspection: Some(payload.new_years_since_inspection), ..Default::default() };
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: vec![Din16798VentSystemEdit::patch(index, row.id.clone(), patch)], ..Default::default() })
}
