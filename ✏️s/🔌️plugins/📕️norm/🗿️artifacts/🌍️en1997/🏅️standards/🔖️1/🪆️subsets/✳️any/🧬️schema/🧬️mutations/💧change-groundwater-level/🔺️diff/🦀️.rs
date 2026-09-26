use super::ChangeGroundwaterLevel;
use crate::diff::En1997Diff;
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeGroundwaterLevel, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_groundwater_level.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "groundwater level must be finite", Vec::<String>::new());
    }
    if base.groundwater_level == payload.new_groundwater_level {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "gwl unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { groundwater_level: Some(payload.new_groundwater_level), ..Default::default() })
}
