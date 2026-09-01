//! ↩️ `rename-object` — undo restores the base-state name; missing id ⇒ `Vec::new()`.

use super::RenameObject;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RenameObject, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::RenameObject(RenameObject { id: payload.id.clone(), new_name: object.name.clone() })]
}
//#endregion 🔖️Inverse
