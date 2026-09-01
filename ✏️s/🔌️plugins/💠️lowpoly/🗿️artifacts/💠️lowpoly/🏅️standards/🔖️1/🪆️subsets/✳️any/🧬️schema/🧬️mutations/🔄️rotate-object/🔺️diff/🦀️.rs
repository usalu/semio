//! 🔺️ `rotate-object` — sparse diff construction: whole-transform patch with only `rotation` changed.
//! Error `target-missing` when absent, Warning `no-op` when already at that rotation, Fatal
//! `invariant` when the rotation is non-finite.

use super::RotateObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot, LowpolyTransform};

//#region 🔖️Diff
pub fn diff(payload: &RotateObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(existing) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_rotation.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Object \"{}\" rotation must be finite, got {:?}.", payload.id, payload.new_rotation), [payload.id.clone()]);
    }
    if existing.transform.rotation == payload.new_rotation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" is already at rotation {:?}.", payload.id, payload.new_rotation));
    }
    let transform = LowpolyTransform { rotation: payload.new_rotation, ..existing.transform.clone() };
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { transform: Some(transform), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
