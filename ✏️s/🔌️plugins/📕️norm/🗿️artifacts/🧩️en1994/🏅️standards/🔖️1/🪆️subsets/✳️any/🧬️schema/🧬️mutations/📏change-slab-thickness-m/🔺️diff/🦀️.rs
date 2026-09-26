//! Diff for `change-slab-thickness-m`.
use super::ChangeSlabThicknessM;
use crate::diff::En1994SlabList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeSlabThicknessM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_concrete_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(slab) = base.slabs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "slab missing", [payload.index.to_string()]);
    };
    if slab.concrete_thickness_m == payload.new_concrete_thickness_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut slabs = base.slabs.clone();
    slabs[payload.index].concrete_thickness_m = payload.new_concrete_thickness_m;
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabList { values: slabs }), ..Default::default() })
}
