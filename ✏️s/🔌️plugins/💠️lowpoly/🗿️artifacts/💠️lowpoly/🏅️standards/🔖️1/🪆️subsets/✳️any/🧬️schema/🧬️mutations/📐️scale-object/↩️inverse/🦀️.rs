//! ↩️ `scale-object` — undo restores the base-state scale; missing id ⇒ `Vec::new()`.

use super::ScaleObject;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::scale_object::ScaleObject>>(payload: &ScaleObject, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ScaleObject { id: payload.id.clone(), new_scale: object.transform.scale })]

    })())
}
//#endregion 🔖️Inverse
