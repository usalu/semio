use super::InsertFatigueDetail;
use crate::diff::En1993FatigueList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.fatigue_details.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.fatigue_detail.clone());
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: Some(En1993FatigueList { values }), ..Default::default() })
}
