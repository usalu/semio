//! ✏️ `change-building-multi-resisting-systems` diff: patches `multiple_resisting_systems` of the addressed building row.

use crate::change_building_multi_resisting_systems::ChangeBuildingMultiResistingSystems;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingMultiResistingSystems, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.multiple_resisting_systems == payload.new_multiple_resisting_systems {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { multiple_resisting_systems: Some(payload.new_multiple_resisting_systems), ..Default::default() }), ..Default::default() })
}
