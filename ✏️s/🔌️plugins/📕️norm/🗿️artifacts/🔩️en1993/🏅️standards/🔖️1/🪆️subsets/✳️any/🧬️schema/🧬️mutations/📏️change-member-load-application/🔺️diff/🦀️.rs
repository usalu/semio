//! 🔺️ `change-member-load-application` diff: patches `load_application` of the addressed member row.

use crate::change_member_load_application::ChangeMemberLoadApplication;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberLoadApplication, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.load_application == payload.new_load_application {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { load_application: Some(payload.new_load_application), ..Default::default() }), ..Default::default() })
}
