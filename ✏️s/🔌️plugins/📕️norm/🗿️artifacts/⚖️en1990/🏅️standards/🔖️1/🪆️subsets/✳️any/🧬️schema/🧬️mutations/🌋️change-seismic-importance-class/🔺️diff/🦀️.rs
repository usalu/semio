//! 🌋 `change-seismic-importance-class` diff: patches `importance_class` of the addressed seismic action row.

use crate::change_seismic_importance_class::ChangeSeismicImportanceClass;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990SeismicDelta, En1990SeismicPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeSeismicImportanceClass, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.seismics.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Seismic action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.importance_class == payload.new_importance_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The seismic action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { seismics: En1990SeismicDelta::modification(&payload.id, En1990SeismicPatch { importance_class: Some(payload.new_importance_class), ..Default::default() }), ..Default::default() })
}
