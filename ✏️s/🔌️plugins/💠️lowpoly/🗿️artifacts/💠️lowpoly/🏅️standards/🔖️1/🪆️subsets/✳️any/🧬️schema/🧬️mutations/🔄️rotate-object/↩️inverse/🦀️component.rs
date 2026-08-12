//! ↩️ `rotate-object` — undo restores the base-state rotation; missing id ⇒ `Vec::new()`.

use super::mutation::RotateObject;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RotateObject, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::RotateObject(RotateObject { id: payload.id.clone(), new_rotation: object.transform.rotation })]
}
//#endregion 🔖️Inverse
