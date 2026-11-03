//! ✏️ `change-foundation-supported-building-id` diff: patches `supported_building_id` of the addressed foundation row.

use crate::change_foundation_supported_building_id::ChangeFoundationSupportedBuildingId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationSupportedBuildingId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.foundations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.supported_building_id == payload.new_supported_building_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&payload.id, En1998FoundationPatch { supported_building_id: Some(payload.new_supported_building_id.clone()), ..Default::default() }), ..Default::default() })
}
