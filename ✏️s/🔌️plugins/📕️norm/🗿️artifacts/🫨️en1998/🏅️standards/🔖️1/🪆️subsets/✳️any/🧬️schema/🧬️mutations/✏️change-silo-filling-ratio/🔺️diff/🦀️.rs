//! ✏️ `change-silo-filling-ratio` diff: patches `filling_ratio` of the addressed silo row.

use crate::change_silo_filling_ratio::ChangeSiloFillingRatio;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SiloDelta, En1998SiloPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSiloFillingRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_filling_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.silos.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("silo {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.filling_ratio == payload.new_filling_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::modification(&payload.id, En1998SiloPatch { filling_ratio: Some(payload.new_filling_ratio), ..Default::default() }), ..Default::default() })
}
