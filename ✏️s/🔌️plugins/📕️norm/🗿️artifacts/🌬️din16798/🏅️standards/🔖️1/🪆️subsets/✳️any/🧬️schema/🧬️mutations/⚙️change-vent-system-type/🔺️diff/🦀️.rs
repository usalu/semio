//! ⚙️ `change-vent-system-type` diff — patches the one field of the row with that id; an id the document does not hold is a `mutation.invariant`.

use crate::change_vent_system_type::ChangeVentSystemType;
use semio_s_artifact_norm_din16798_diff::{Din16798Diff, Din16798VentSystemDelta, Din16798VentSystemPatch};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn diff(payload: &ChangeVentSystemType, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    let patch = Din16798VentSystemPatch { system_type: Some(payload.new_system_type.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Din16798Diff { vent_systems: Din16798VentSystemDelta::modification(&row.id, patch), ..Default::default() })
}
