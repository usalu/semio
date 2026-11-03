//! 🔺️ `change-built-up-planes` diff: patches `planes` of the addressed built-up member row.

use crate::change_built_up_planes::ChangeBuiltUpPlanes;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BuiltUpMemberDelta, En1993BuiltUpMemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBuiltUpPlanes, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.built_up_members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "built-up", Vec::<String>::new());
    };
    if row.planes == payload.new_planes {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The built-up member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { built_up_members: En1993BuiltUpMemberDelta::modification(payload.id.clone(), En1993BuiltUpMemberPatch { planes: Some(payload.new_planes), ..Default::default() }), ..Default::default() })
}
