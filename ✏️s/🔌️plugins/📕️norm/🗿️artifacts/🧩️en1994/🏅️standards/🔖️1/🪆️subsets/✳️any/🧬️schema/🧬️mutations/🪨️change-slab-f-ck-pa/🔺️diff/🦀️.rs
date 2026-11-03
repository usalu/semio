//! 🔺️ Diff for `change-slab-f-ck-pa`.
use crate::change_slab_f_ck_pa::ChangeSlabFCkPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994SlabsRows, En1994SlabsPatch};

pub fn diff(payload: &ChangeSlabFCkPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slab {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_f_ck_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(slab) = base.slabs.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "slab missing", [index.to_string()]);
    };
    if (slab.f_ck_pa - payload.new_f_ck_pa).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        slabs: Some(En1994SlabsRows::modification(&payload.id, En1994SlabsPatch { f_ck_pa: Some(payload.new_f_ck_pa), ..Default::default() })),
        ..Default::default()
    })
}
