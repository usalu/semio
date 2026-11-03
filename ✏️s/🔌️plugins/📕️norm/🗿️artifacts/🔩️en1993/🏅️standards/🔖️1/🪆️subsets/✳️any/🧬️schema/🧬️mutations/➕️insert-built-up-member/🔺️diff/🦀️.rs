//! ➕️ `insert-built-up-member` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_built_up_member::InsertBuiltUpMember;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BuiltUpMemberDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertBuiltUpMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.built_up_members.iter().any(|existing| existing.id == payload.built_up_member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Built-up member id {} already exists.", payload.built_up_member.id), [payload.built_up_member.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.built_up_members.len());
    protocol::MutationOutcome::new(En1993Diff { built_up_members: En1993BuiltUpMemberDelta::insertion(index, payload.built_up_member.clone()), ..Default::default() })
}
