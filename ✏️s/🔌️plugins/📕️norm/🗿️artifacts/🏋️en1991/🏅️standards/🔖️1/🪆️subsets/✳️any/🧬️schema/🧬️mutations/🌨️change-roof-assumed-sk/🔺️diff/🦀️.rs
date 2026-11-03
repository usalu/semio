//! 🌨️ `change-roof-assumed-sk` diff: patches `assumed_sk` of the addressed roof row.

use crate::change_roof_assumed_sk::ChangeRoofAssumedSk;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofDelta, En1991RoofPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_sk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.roofs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_sk == payload.new_assumed_sk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::modification(&payload.id, En1991RoofPatch { assumed_sk: Some(payload.new_assumed_sk), ..Default::default() }), ..Default::default() })
}
