//! 🔺️ `change-material-ductility-elongation` diff: patches `elongation` of the addressed material_ductility block row.

use crate::change_material_ductility_elongation::ChangeMaterialDuctilityElongation;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDuctilityDelta, En1993MaterialDuctilityPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMaterialDuctilityElongation, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_elongation.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.material_ductility.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "material-ductility", Vec::<String>::new());
    };
    if row.elongation == payload.new_elongation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The material_ductility block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { material_ductility: En1993MaterialDuctilityDelta::modification(payload.id.clone(), En1993MaterialDuctilityPatch { elongation: Some(payload.new_elongation), ..Default::default() }), ..Default::default() })
}
