//! 🔺️ `change-built-up-back-to-back` diff: patches `back_to_back` of the addressed built-up member row.

use crate::change_built_up_back_to_back::ChangeBuiltUpBackToBack;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BuiltUpMemberDelta, En1993BuiltUpMemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBuiltUpBackToBack, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.built_up_members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "built-up", Vec::<String>::new());
    };
    if row.back_to_back == payload.new_back_to_back {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The built-up member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { built_up_members: En1993BuiltUpMemberDelta::modification(payload.id.clone(), En1993BuiltUpMemberPatch { back_to_back: Some(payload.new_back_to_back), ..Default::default() }), ..Default::default() })
}
