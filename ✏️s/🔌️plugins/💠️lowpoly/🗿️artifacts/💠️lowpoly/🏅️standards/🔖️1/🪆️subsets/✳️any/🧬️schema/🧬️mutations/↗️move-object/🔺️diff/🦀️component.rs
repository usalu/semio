//! 🔺️ `move-object` — sparse diff construction: whole-transform patch with only `position` changed
//! (storage only supports a whole `LowpolyTransform` slot, so the untouched fields are read from base).
//! Error `target-missing` when absent, Warning `no-op` when already at that position, Fatal
//! `invariant` when the position is non-finite.

use super::mutation::MoveObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot, LowpolyTransform};

//#region 🔖️Diff
pub fn diff(payload: &MoveObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(existing) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_position.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Object \"{}\" position must be finite, got {:?}.", payload.id, payload.new_position), [payload.id.clone()]);
    }
    if existing.transform.position == payload.new_position {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" is already at position {:?}.", payload.id, payload.new_position));
    }
    let transform = LowpolyTransform { position: payload.new_position, ..existing.transform.clone() };
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { transform: Some(transform), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
