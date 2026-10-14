//! ↩️ `move-object` — undo restores the base-state position; missing id ⇒ `Vec::new()`.

use super::MoveObject;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::move_object::MoveObject>>(payload: &MoveObject, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(MoveObject { id: payload.id.clone(), new_position: object.transform.position })]

    })())
}
//#endregion 🔖️Inverse
