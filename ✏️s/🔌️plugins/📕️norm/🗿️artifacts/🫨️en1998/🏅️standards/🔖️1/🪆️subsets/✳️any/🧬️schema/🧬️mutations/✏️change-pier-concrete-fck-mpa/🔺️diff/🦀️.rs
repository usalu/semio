//! ✏️ `change-pier-concrete-fck-mpa` diff: patches `concrete_fck_mpa` of the addressed pier row.

use crate::change_pier_concrete_fck_mpa::ChangePierConcreteFckMpa;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierDelta, En1998PierPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangePierConcreteFckMpa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_concrete_fck_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.concrete_fck_mpa == payload.new_concrete_fck_mpa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pier already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { piers: En1998PierDelta::modification(&payload.id, En1998PierPatch { concrete_fck_mpa: Some(payload.new_concrete_fck_mpa), ..Default::default() }), ..Default::default() })
}
