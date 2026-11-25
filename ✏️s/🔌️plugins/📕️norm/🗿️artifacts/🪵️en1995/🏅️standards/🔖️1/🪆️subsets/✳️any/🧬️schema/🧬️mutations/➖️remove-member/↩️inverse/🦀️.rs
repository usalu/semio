use crate::remove_member::RemoveMember;
use crate::insert_member;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveMember, base: &En1995Snapshot) -> Option<crate::insert_member::InsertMember> {
    if payload.index >= base.members.len() { return None; }
    Some(insert_member::InsertMember { index: payload.index, member: base.members[payload.index].clone() })
}
