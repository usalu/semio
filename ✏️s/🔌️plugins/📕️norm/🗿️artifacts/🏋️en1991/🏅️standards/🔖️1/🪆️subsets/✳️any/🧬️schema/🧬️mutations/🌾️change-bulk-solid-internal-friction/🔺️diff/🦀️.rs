//! 🌾 `change-bulk-solid-internal-friction` diff: patches `internal_friction` of the addressed bulk solid row.

use crate::change_bulk_solid_internal_friction::ChangeBulkSolidInternalFriction;
use semio_s_artifact_norm_en1991_diff::{En1991BulkSolidDelta, En1991BulkSolidPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeBulkSolidInternalFriction, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_internal_friction.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bulk_solids.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bulk solid {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.internal_friction == payload.new_internal_friction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bulk solid already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bulk_solids: En1991BulkSolidDelta::modification(&payload.id, En1991BulkSolidPatch { internal_friction: Some(payload.new_internal_friction), ..Default::default() }), ..Default::default() })
}
