//! ↩️ `change-object-smooth-shading` — undo restores the base-state flag; missing id ⇒ `Vec::new()`.

use super::ChangeObjectSmoothShading;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeObjectSmoothShading, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![LowpolyMutation::ChangeObjectSmoothShading(ChangeObjectSmoothShading { id: payload.id.clone(), new_smooth_shading: object.smooth_shading })]
}
//#endregion 🔖️Inverse
