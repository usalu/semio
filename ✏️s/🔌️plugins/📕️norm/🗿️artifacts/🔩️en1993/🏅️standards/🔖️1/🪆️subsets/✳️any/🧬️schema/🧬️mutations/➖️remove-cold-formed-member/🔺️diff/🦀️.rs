//! ➖️ `remove-cold-formed-member` diff — removes the row at the index.

use crate::remove_cold_formed_member::RemoveColdFormedMember;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ColdFormedMemberDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.cold_formed_members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("cold-formed-member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::removal(&base.cold_formed_members, payload.index), ..Default::default() })
}
