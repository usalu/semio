//! ✏️ `change-silo-q-nominal` diff: patches `q_nominal` of the addressed silo row.

use crate::change_silo_q_nominal::ChangeSiloQNominal;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SiloDelta, En1998SiloPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSiloQNominal, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_q_nominal.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.silos.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("silo {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.q_nominal == payload.new_q_nominal {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::modification(&payload.id, En1998SiloPatch { q_nominal: Some(payload.new_q_nominal), ..Default::default() }), ..Default::default() })
}
