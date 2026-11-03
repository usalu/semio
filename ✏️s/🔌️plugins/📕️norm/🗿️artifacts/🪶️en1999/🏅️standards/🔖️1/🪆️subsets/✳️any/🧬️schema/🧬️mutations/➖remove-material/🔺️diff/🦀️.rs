//! 🔺️ `remove-material` diff.

use crate::remove_material::RemoveMaterial;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MaterialsRows};

pub fn diff(payload: &RemoveMaterial, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.materials.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown material id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { materials: Some(En1999MaterialsRows::removal(&base.materials, index)), ..Default::default() })
}
