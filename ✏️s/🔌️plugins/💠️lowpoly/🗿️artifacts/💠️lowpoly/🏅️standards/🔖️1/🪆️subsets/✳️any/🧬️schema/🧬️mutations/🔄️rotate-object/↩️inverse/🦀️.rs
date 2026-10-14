//! ↩️ `rotate-object` — undo restores the base-state rotation; missing id ⇒ `Vec::new()`.

use super::RotateObject;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rotate_object::RotateObject>>(payload: &RotateObject, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(RotateObject { id: payload.id.clone(), new_rotation: object.transform.rotation })]

    })())
}
//#endregion 🔖️Inverse
