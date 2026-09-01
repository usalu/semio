//! 🔺️ `delete-object` — sparse diff construction (delegates to the existing objects-remove field-delta
//! constructor); Error `target-missing` when the object is already absent.

use super::DeleteObject;
use crate::artifacts::lowpoly::diff::diff_objects_remove;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if !base.objects.iter().any(|object| object.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_objects_remove(payload.id.clone()))
}
//#endregion 🔖️Diff
