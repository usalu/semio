//! ✏️ `change-accelerogram-building-id` diff: patches `building_id` of the addressed accelerogram row.

use crate::change_accelerogram_building_id::ChangeAccelerogramBuildingId;
use semio_s_artifact_norm_en1998_diff::{En1998AccelerogramDelta, En1998AccelerogramPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAccelerogramBuildingId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.accelerograms.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("accelerogram {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.building_id == payload.new_building_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accelerogram already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { accelerograms: En1998AccelerogramDelta::modification(&payload.id, En1998AccelerogramPatch { building_id: Some(payload.new_building_id.clone()), ..Default::default() }), ..Default::default() })
}
