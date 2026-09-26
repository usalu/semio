//! 🔺️ `change-cellar-area` diff.
use super::ChangeCellarArea;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeCellarArea, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.cellar_area_m2 == payload.new_cellar_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { cellar_area_m2: Some(payload.new_cellar_area_m2), ..Default::default() })
}
