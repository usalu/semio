//! 🪚 `remove-member` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990MemberDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveMember, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.members.len() {
        return MutationOutcome::error("mutation.target-missing", "members index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { members: En1990MemberDelta::removal(&base.members, payload.index), ..En1990Diff::default() })
}
