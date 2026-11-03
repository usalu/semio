//! ✏️ `change-foundation-base-friction-deg` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_foundation_base_friction_deg::ChangeFoundationBaseFrictionDeg;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationBaseFrictionDeg, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.foundations.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "foundation", Vec::<String>::new());
    };
    let patch = En1998FoundationPatch { base_friction_angle_deg: Some(payload.new_base_friction_angle_deg), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&row.id, patch), ..Default::default() })
}
