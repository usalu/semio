//! 🏞️ `change-vent-oda-class` diff — patches the one field of the row with that id; an id the document does not hold is a `mutation.invariant`.

use super::ChangeVentOdaClass;
use crate::diff::{Din16798Diff, Din16798VentSystemDelta, Din16798VentSystemPatch};
use crate::Din16798Snapshot;

pub fn diff(payload: &ChangeVentOdaClass, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    let patch = Din16798VentSystemPatch { oda_class: Some(payload.new_oda_class.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Din16798VentSystemDelta::modification(&row.id, patch), ..Default::default() })
}
