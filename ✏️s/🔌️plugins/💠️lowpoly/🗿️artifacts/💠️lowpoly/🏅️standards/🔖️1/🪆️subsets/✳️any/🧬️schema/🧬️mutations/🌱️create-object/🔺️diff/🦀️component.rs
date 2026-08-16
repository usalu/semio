//! 🔺️ `create-object` — sparse diff construction (delegates to the existing objects-add field-delta
//! constructor); Fatal `duplicate-id` guards against overwriting an existing object.

use super::mutation::CreateObject;
use crate::artifacts::lowpoly::diff::diff_objects_add;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if base.objects.iter().any(|object| object.id == payload.object.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An object with id \"{}\" already exists.", payload.object.id), [payload.object.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_objects_add(payload.index, payload.object.clone(), base))
}
//#endregion 🔖️Diff
