//! ✏️ `change-bridge-pier-shear-rd-n` diff: patches `pier_shear_rd_n` of the addressed bridge row.

use crate::change_bridge_pier_shear_rd_n::ChangeBridgePierShearRdN;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgePierShearRdN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_pier_shear_rd_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.pier_shear_rd_n == payload.new_pier_shear_rd_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.id, En1998BridgePatch { pier_shear_rd_n: Some(payload.new_pier_shear_rd_n), ..Default::default() }), ..Default::default() })
}
