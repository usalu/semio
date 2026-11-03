//! ✏️ `change-diaphragm-storey-id` diff: patches `storey_id` of the addressed diaphragm row.

use crate::change_diaphragm_storey_id::ChangeDiaphragmStoreyId;
use semio_s_artifact_norm_en1998_diff::{En1998DiaphragmDelta, En1998DiaphragmPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeDiaphragmStoreyId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.diaphragms.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("diaphragm {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.storey_id == payload.new_storey_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The diaphragm already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { diaphragms: En1998DiaphragmDelta::modification(&payload.id, En1998DiaphragmPatch { storey_id: Some(payload.new_storey_id.clone()), ..Default::default() }), ..Default::default() })
}
