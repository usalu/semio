//! Diff for `change-steel-fy-pa`.
use super::ChangeSteelFYPa;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeSteelFYPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_steel_f_y_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.steel_f_y_pa == payload.new_steel_f_y_pa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { steel_f_y_pa: Some(payload.new_steel_f_y_pa), ..Default::default() })
}
