//! ↩️ `create-object` — undo is `delete-object`, unless `base` already had this id (then `create`
//! was a no-op and there's nothing to undo).

use super::CreateObject;
use crate::delete_object;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_object::DeleteObject>>(payload: &CreateObject, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.objects.iter().any(|object| object.id == payload.object.id) {
        return Vec::new();
    }
    vec![Op::from(delete_object::DeleteObject { id: payload.object.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
