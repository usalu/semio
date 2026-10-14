//! 🔺️ `add-member` diff.

use crate::add_member::AddMember;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersRows};

pub fn diff(payload: &AddMember, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.members.iter().any(|existing| existing.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), [payload.member.id.clone()]);
    }
    if (payload.index as usize) > base.members.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.members.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::insertion(payload.index as usize, payload.member.clone())), ..Default::default() })
}
