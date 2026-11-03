//! 🔺️ `change-material-ductility-material-id` diff: patches `material_id` of the addressed material_ductility block row.

use crate::change_material_ductility_material_id::ChangeMaterialDuctilityMaterialId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDuctilityDelta, En1993MaterialDuctilityPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMaterialDuctilityMaterialId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.material_ductility.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "material-ductility", Vec::<String>::new());
    };
    if row.material_id == payload.new_material_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The material_ductility block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { material_ductility: En1993MaterialDuctilityDelta::modification(payload.id.clone(), En1993MaterialDuctilityPatch { material_id: Some(payload.new_material_id.clone()), ..Default::default() }), ..Default::default() })
}
