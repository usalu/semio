use super::RemoveFatigueDetail;
use crate::diff::En1993FatigueList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.fatigue_details.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fatigue-detail index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.fatigue_details.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: Some(En1993FatigueList { values }), ..Default::default() })
}
