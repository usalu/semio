use super::InsertFatigueDetail;
use crate::diff::En1993FatigueList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fatigue_details.iter().any(|existing| existing.id == payload.fatigue_detail.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue detail id {} already exists.", payload.fatigue_detail.id), [payload.fatigue_detail.id.clone()]);
    }
    let mut values = base.fatigue_details.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.fatigue_detail.clone());
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: Some(En1993FatigueList { values }), ..Default::default() })
}
