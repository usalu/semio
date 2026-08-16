//! 🔺️ `change-object-smooth-shading` — sparse diff construction: one-field object patch. Error
//! `target-missing` when absent, Warning `no-op` when the flag is unchanged.

use super::mutation::ChangeObjectSmoothShading;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeObjectSmoothShading, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(existing) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.smooth_shading == payload.new_smooth_shading {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" smooth shading is already {}.", payload.id, payload.new_smooth_shading));
    }
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { smooth_shading: Some(payload.new_smooth_shading), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
