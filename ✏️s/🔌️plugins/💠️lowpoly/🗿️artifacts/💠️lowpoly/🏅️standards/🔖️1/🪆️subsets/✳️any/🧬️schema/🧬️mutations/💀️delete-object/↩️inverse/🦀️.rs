//! ↩️ `delete-object` — undo re-`create-object`s the captured object at its base-state index;
//! missing id ⇒ `Vec::new()`.

use super::DeleteObject;
use crate::create_object;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_object::CreateObject>>(payload: &DeleteObject, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(index) = base.objects.iter().position(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(create_object::CreateObject { index, object: base.objects[index].clone() })]

    })())
}
//#endregion 🔖️Inverse
