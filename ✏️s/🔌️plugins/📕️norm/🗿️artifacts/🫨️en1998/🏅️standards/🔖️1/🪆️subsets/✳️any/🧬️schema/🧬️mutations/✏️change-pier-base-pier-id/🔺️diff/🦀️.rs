//! ✏️ `change-pier-base-pier-id` diff: patches `pier_id` of the addressed pier_base row.

use crate::change_pier_base_pier_id::ChangePierBasePierId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierBaseDelta, En1998PierBasePatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangePierBasePierId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.pier_bases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier_base {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.pier_id == payload.new_pier_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pier_base already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { pier_bases: En1998PierBaseDelta::modification(&payload.id, En1998PierBasePatch { pier_id: Some(payload.new_pier_id.clone()), ..Default::default() }), ..Default::default() })
}
