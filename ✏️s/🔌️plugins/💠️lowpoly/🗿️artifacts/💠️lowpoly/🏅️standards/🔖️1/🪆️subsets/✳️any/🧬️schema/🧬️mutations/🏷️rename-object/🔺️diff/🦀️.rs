//! 🔺️ `rename-object` — sparse diff construction: one-field object patch on `name`. Error
//! `target-missing` when absent, Warning `no-op` when the new name equals the old (object `name` is a
//! non-unique display string, not a key, so no `duplicate-id` case applies here).

use super::RenameObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(existing) = base.objects.iter().find(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { name: Some(payload.new_name.clone()), ..LowpolyObjectPatch::default() }))
}
//#endregion 🔖️Diff
