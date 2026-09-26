//! Diff for `change-fatigue-detail`.
use super::ChangeFatigueDetail;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeFatigueDetail, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.new_fatigue_detail.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.fatigue_detail == payload.new_fatigue_detail {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { fatigue_detail: Some(payload.new_fatigue_detail.clone()), ..Default::default() })
}
