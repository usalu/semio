//! 🏠 `change-roof-ce` diff: patches `c_e` of the addressed roof row.

use crate::change_roof_c_e::ChangeRoofCE;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofDelta, En1991RoofPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofCE, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_c_e.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.roofs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Roof {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.c_e == payload.new_c_e {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::modification(&payload.id, En1991RoofPatch { c_e: Some(payload.new_c_e), ..Default::default() }), ..Default::default() })
}
