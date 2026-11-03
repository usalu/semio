//! 🌀 `change-dynamic-log-decrement` diff: patches `log_decrement` of the addressed dynamic element row.

use crate::change_dynamic_log_decrement::ChangeDynamicLogDecrement;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DynamicElementDelta, En1991DynamicElementPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeDynamicLogDecrement, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_log_decrement.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.dynamic_elements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Dynamic element {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.log_decrement == payload.new_log_decrement {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The dynamic element already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { dynamic_elements: En1991DynamicElementDelta::modification(&payload.id, En1991DynamicElementPatch { log_decrement: Some(payload.new_log_decrement), ..Default::default() }), ..Default::default() })
}
