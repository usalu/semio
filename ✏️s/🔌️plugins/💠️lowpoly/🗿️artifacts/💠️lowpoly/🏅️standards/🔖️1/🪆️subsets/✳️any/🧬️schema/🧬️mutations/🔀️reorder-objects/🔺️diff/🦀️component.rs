//! 🔺️ `reorder-objects` — sparse diff construction (delegates to the existing objects-move field-delta
//! constructor). Error `target-missing` when the id is unknown, Warning `no-op` when the resulting
//! order is unchanged.

use super::mutation::ReorderObjects;
use crate::artifacts::lowpoly::diff::diff_objects_move;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderObjects, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if !base.objects.iter().any(|object| object.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let original: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
    let mut reordered = original.clone();
    if let Some(from) = reordered.iter().position(|id| id == &payload.id) {
        let moved = reordered.remove(from);
        let at = payload.to_index.min(reordered.len());
        reordered.insert(at, moved);
    }
    if reordered == original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(diff_objects_move(&payload.id, payload.to_index, base))
}
//#endregion 🔖️Diff
