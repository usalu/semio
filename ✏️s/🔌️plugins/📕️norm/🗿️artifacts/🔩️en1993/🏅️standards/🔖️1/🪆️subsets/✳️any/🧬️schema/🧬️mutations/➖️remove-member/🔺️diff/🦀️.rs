//! ➖️ `remove-member` diff — removes the row at the index.

use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::removal(&base.members, payload.index), ..Default::default() })
}
