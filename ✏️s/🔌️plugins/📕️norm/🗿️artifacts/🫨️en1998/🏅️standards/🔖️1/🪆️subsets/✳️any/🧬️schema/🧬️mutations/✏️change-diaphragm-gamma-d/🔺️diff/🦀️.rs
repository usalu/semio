//! ✏️ `change-diaphragm-gamma-d` diff: patches `gamma_d` of the addressed diaphragm row.

use crate::change_diaphragm_gamma_d::ChangeDiaphragmGammaD;
use semio_s_artifact_norm_en1998_diff::{En1998DiaphragmDelta, En1998DiaphragmPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeDiaphragmGammaD, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_gamma_d.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.diaphragms.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("diaphragm {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.gamma_d == payload.new_gamma_d {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The diaphragm already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { diaphragms: En1998DiaphragmDelta::modification(&payload.id, En1998DiaphragmPatch { gamma_d: Some(payload.new_gamma_d), ..Default::default() }), ..Default::default() })
}
