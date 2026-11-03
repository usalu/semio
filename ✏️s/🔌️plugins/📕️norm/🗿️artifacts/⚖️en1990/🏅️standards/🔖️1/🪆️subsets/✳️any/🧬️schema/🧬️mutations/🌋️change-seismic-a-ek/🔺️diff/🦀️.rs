//! 🌋 `change-seismic-a-ek` diff: patches `a_ek` of the addressed seismic action row.

use crate::change_seismic_a_ek::ChangeSeismicAEk;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990SeismicDelta, En1990SeismicPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeSeismicAEk, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_a_ek.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.seismics.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Seismic action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.a_ek == payload.new_a_ek {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The seismic action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { seismics: En1990SeismicDelta::modification(&payload.id, En1990SeismicPatch { a_ek: Some(payload.new_a_ek), ..Default::default() }), ..Default::default() })
}
