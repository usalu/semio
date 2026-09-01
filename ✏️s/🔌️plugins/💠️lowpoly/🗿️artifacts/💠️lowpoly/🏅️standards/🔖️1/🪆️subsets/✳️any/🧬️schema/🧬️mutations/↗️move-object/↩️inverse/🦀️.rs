//! ↩️ `move-object` — undo restores the base-state position; missing id ⇒ `Vec::new()`.

use super::MoveObject;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &MoveObject, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::MoveObject(MoveObject { id: payload.id.clone(), new_position: object.transform.position })]
}
//#endregion 🔖️Inverse
