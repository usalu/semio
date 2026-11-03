//! ✏️ `change-pier-steel-fyk-mpa` diff: patches `steel_fyk_mpa` of the addressed pier row.

use crate::change_pier_steel_fyk_mpa::ChangePierSteelFykMpa;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierDelta, En1998PierPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangePierSteelFykMpa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_steel_fyk_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.steel_fyk_mpa == payload.new_steel_fyk_mpa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pier already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { piers: En1998PierDelta::modification(&payload.id, En1998PierPatch { steel_fyk_mpa: Some(payload.new_steel_fyk_mpa), ..Default::default() }), ..Default::default() })
}
