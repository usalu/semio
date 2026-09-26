//! Diff for `change-insulation-thickness-m`.
use super::ChangeInsulationThicknessM;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeInsulationThicknessM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_insulation_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.insulation_thickness_m == payload.new_insulation_thickness_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { insulation_thickness_m: Some(payload.new_insulation_thickness_m), ..Default::default() })
}
