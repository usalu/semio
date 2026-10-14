//! ➕️ `insert-member` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_member::InsertMember;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.members.iter().any(|existing| existing.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), [payload.member.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.members.len());
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::insertion(index, payload.member.clone()), ..Default::default() })
}
