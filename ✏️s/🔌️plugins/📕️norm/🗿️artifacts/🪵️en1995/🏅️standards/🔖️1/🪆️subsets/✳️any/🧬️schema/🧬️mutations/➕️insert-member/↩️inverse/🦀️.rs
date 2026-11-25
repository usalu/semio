use crate::insert_member::InsertMember;
use crate::remove_member;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertMember, base: &En1995Snapshot) -> Option<crate::remove_member::RemoveMember> {
    let at = payload.index.min(base.members.len());
    Some(remove_member::RemoveMember { index: at })
}
