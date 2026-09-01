//! ↩️ `delete-object` — undo re-`create-object`s the captured object at its base-state index;
//! missing id ⇒ `Vec::new()`.

use super::DeleteObject;
use crate::artifacts::lowpoly::mutations::create_object;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteObject, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(index) = base.objects.iter().position(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::CreateObject(create_object::CreateObject { index, object: base.objects[index].clone() })]
}
//#endregion 🔖️Inverse
