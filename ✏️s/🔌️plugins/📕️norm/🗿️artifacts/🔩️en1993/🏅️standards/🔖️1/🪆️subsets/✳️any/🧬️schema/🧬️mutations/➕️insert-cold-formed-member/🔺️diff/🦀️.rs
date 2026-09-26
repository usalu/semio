use super::InsertColdFormedMember;
use crate::diff::En1993ColdFormedList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.cold_formed_members.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.cold_formed_member.clone());
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: Some(En1993ColdFormedList { values }), ..Default::default() })
}
