//! ↩️ `reorder-objects` — undo reorders back to the base-state index; missing id ⇒ `Vec::new()`.

use super::mutation::ReorderObjects;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ReorderObjects, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(original_index) = base.objects.iter().position(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ReorderObjects(ReorderObjects { id: payload.id.clone(), to_index: original_index })]
}
//#endregion 🔖️Inverse
