//! 🆔️ `change-vent-id` diff — replaces the row by a copy under the new id at the same position and points every zone served by the system at the new id; an id the document does not hold is a `mutation.invariant`, an id it already holds or that a zone already names a `mutation.duplicate-id`.

use crate::change_vent_id::ChangeVentId;
use semio_s_artifact_norm_din16798_diff::{Din16798Diff, Din16798VentSystemDelta, Din16798ZoneDelta, Din16798ZonePatch};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn diff(payload: &ChangeVentId, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(index) = base.vent_systems.iter().position(|row| row.id == payload.vent_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    };
    if payload.new_id == payload.vent_id {
        return protocol::MutationOutcome::new(Din16798Diff::default());
    }
    if base.vent_systems.iter().any(|row| row.id == payload.new_id) || base.zones.iter().any(|zone| zone.vent_system_id == payload.new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A ventilation system with id '{}' already exists or is already named by a zone.", payload.new_id), [payload.new_id.clone()]);
    }
    let mut renamed = base.vent_systems[index].clone();
    renamed.id = payload.new_id.clone();
    let mut vent_systems = Din16798VentSystemDelta::removal(&base.vent_systems, index);
    vent_systems.absorb(Din16798VentSystemDelta::insertion(index, renamed));
    let mut zones = Din16798ZoneDelta::default();
    for zone in base.zones.iter().filter(|zone| zone.vent_system_id == payload.vent_id) {
        zones.absorb(Din16798ZoneDelta::modification(&zone.id, Din16798ZonePatch { vent_system_id: Some(payload.new_id.clone()), ..Default::default() }));
    }
    protocol::MutationOutcome::new(Din16798Diff { vent_systems, zones, ..Default::default() })
}
