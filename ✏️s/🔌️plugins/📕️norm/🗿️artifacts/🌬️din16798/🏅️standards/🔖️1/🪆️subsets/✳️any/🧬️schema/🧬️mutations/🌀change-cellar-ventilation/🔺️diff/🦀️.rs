//! 🔺️ `change-cellar-ventilation` diff.
use super::ChangeCellarVentilation;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeCellarVentilation, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.cellar_ventilation_m3_h == payload.new_cellar_ventilation_m3_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { cellar_ventilation_m3_h: Some(payload.new_cellar_ventilation_m3_h), ..Default::default() })
}
