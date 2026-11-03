//! 🌀 `change-dynamic-assumed-cscd` diff: patches `assumed_cscd` of the addressed dynamic element row.

use crate::change_dynamic_assumed_cscd::ChangeDynamicAssumedCscd;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DynamicElementDelta, En1991DynamicElementPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeDynamicAssumedCscd, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_cscd.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.dynamic_elements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Dynamic element {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_cscd == payload.new_assumed_cscd {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The dynamic element already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { dynamic_elements: En1991DynamicElementDelta::modification(&payload.id, En1991DynamicElementPatch { assumed_cscd: Some(payload.new_assumed_cscd), ..Default::default() }), ..Default::default() })
}
