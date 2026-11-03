//! ➖️ `remove-built-up-member` diff — removes the row at the index.

use crate::remove_built_up_member::RemoveBuiltUpMember;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BuiltUpMemberDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveBuiltUpMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.built_up_members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("built-up-member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { built_up_members: En1993BuiltUpMemberDelta::removal(&base.built_up_members, payload.index), ..Default::default() })
}
