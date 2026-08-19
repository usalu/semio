//! ↩️ `create-object` — undo is `delete-object`, unless `base` already had this id (then `create`
//! was a no-op and there's nothing to undo).

use super::mutation::CreateObject;
use crate::artifacts::lowpoly::mutations::delete_object;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateObject, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    if base.objects.iter().any(|object| object.id == payload.object.id) {
        return Vec::new();
    }
    vec![LowpolyMutation::DeleteObject(delete_object::mutation::DeleteObject { id: payload.object.id.clone() })]
}
//#endregion 🔖️Inverse
