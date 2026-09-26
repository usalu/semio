use super::RemoveColdFormedMember;
use crate::diff::En1993ColdFormedList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveColdFormedMember, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.cold_formed_members.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("cold-formed-member index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.cold_formed_members.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: Some(En1993ColdFormedList { values }), ..Default::default() })
}
