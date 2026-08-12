//! ↩️ `scale-object` — undo restores the base-state scale; missing id ⇒ `Vec::new()`.

use super::mutation::ScaleObject;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleObject, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ScaleObject(ScaleObject { id: payload.id.clone(), new_scale: object.transform.scale })]
}
//#endregion 🔖️Inverse
