//! 🔺️ Diff for `change-slab-thickness-m`.
use crate::change_slab_thickness_m::ChangeSlabThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994SlabsRows, En1994SlabsPatch};

pub fn diff(payload: &ChangeSlabThicknessM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_concrete_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(slab) = base.slabs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "slab missing", [payload.index.to_string()]);
    };
    if slab.concrete_thickness_m == payload.new_concrete_thickness_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        slabs: Some(En1994SlabsRows { modified: vec![En1994SlabsPatch { index: payload.index, concrete_thickness_m: Some(payload.new_concrete_thickness_m), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
