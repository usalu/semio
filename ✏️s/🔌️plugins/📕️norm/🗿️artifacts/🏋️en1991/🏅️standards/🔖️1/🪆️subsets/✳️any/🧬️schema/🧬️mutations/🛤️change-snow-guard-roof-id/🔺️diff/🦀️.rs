//! 🛤 `change-snow-guard-roof-id` diff: patches `roof_id` of the addressed fatigue roof_id row.

use crate::change_snow_guard_roof_id::ChangeSnowGuardRoofId;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SnowGuardDelta, En1991SnowGuardPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSnowGuardRoofId, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.snow_guards.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue roof_id {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.roof_id == payload.new_roof_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue roof_id already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { snow_guards: En1991SnowGuardDelta::modification(&payload.id, En1991SnowGuardPatch { roof_id: Some(payload.new_roof_id.clone()), ..Default::default() }), ..Default::default() })
}
