//! 🔺️ `reorder-objects` — sparse diff construction (delegates to the existing objects-move field-delta
//! constructor). Error `target-missing` when the id is unknown, Warning `no-op` when the resulting
//! order is unchanged.

use super::ReorderObjects;
use crate::diff::diff_objects_move;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderObjects, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(from) = base.objects.iter().position(|object| object.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if from == payload.to_index.min(base.objects.len() - 1) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Object \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(diff_objects_move(&payload.id, payload.to_index, base))
}
//#endregion 🔖️Diff
