//! ✏️ `change-system-system-type` diff: patches `system_type` of the addressed system row.

use crate::change_system_system_type::ChangeSystemSystemType;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998SystemDelta, En1998SystemPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSystemSystemType, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.systems.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("system {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.system_type == payload.new_system_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The system already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { systems: En1998SystemDelta::modification(&payload.id, En1998SystemPatch { system_type: Some(payload.new_system_type), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
