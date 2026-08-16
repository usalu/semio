//! 🔺️ `scale-object` — sparse diff construction: whole-transform patch with only `scale` changed.
//! Error `target-missing` when absent, Warning `no-op` when already at that scale, Fatal `invariant`
//! when a scale component is non-finite or non-positive.

use super::mutation::ScaleObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot, LowpolyTransform};

//#region 🔖️Diff
pub fn diff(payload: &ScaleObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(existing) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_scale.iter().any(|value| !value.is_finite() || *value <= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Object \"{}\" scale must be finite and positive, got {:?}.", payload.id, payload.new_scale), [payload.id.clone()]);
    }
    if existing.transform.scale == payload.new_scale {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" is already at scale {:?}.", payload.id, payload.new_scale));
    }
    let transform = LowpolyTransform { scale: payload.new_scale, ..existing.transform.clone() };
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { transform: Some(transform), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
