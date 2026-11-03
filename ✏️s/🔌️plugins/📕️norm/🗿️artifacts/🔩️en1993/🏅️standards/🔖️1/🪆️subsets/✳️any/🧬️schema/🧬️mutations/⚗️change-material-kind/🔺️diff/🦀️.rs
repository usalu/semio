//! 🔺️ `change-material-kind` diff: patches `kind` of the addressed material row.

use crate::change_material_kind::ChangeMaterialKind;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDelta, En1993MaterialPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMaterialKind, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.materials.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "material", Vec::<String>::new());
    };
    if row.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The material already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { materials: En1993MaterialDelta::modification(payload.id.clone(), En1993MaterialPatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
