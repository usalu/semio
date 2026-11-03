//! 🔺️ `change-material-subgrade` diff: patches `subgrade` of the addressed material row.

use crate::change_material_subgrade::ChangeMaterialSubgrade;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDelta, En1993MaterialPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMaterialSubgrade, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.materials.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "material", Vec::<String>::new());
    };
    if row.subgrade == payload.new_subgrade {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The material already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::modification(payload.id.clone(), En1993MaterialPatch { subgrade: Some(payload.new_subgrade.clone()), ..Default::default() }), ..Default::default() })
}
