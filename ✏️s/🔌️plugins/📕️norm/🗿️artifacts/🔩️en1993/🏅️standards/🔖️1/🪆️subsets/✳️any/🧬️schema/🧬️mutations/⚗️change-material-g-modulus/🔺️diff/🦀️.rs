//! 🔺️ `change-material-g-modulus` diff: patches `g_modulus` of the addressed material row.

use crate::change_material_g_modulus::ChangeMaterialGModulus;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDelta, En1993MaterialPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMaterialGModulus, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_g_modulus.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.materials.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "material", Vec::<String>::new());
    };
    if row.g_modulus == payload.new_g_modulus {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The material already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::modification(payload.id.clone(), En1993MaterialPatch { g_modulus: Some(payload.new_g_modulus), ..Default::default() }), ..Default::default() })
}
