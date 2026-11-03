//! 📏 `change-load-dispersal-slab-thickness` diff: patches `slab_thickness` of the addressed load dispersal row.

use crate::change_load_dispersal_slab_thickness::ChangeLoadDispersalSlabThickness;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LoadDispersalDelta, En1991LoadDispersalPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLoadDispersalSlabThickness, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_slab_thickness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.load_dispersals.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load dispersal {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.slab_thickness == payload.new_slab_thickness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The load dispersal already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { load_dispersals: En1991LoadDispersalDelta::modification(&payload.id, En1991LoadDispersalPatch { slab_thickness: Some(payload.new_slab_thickness), ..Default::default() }), ..Default::default() })
}
