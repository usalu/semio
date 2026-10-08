//! 🔺️ `delete-object` — sparse diff construction (delegates to the existing objects-remove field-delta
//! constructor); Error `target-missing` when the object is already absent.

use super::DeleteObject;
use crate::diff::diff_objects_remove;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(index) = base.objects.iter().position(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(diff_objects_remove(index, base))
}
//#endregion 🔖️Diff
