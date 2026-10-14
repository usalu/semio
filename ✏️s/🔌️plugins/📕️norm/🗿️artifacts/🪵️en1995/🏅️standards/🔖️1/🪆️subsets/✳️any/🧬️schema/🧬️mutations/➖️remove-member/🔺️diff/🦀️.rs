use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995MemberDelta;
pub fn diff(payload: &RemoveMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Member index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { members: En1995MemberDelta::removal(&base.members, payload.index), ..Default::default() })
}
