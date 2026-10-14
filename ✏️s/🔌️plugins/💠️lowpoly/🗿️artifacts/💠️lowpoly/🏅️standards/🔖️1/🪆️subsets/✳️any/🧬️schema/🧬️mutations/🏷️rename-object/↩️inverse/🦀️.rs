//! ↩️ `rename-object` — undo restores the base-state name; missing id ⇒ `Vec::new()`.

use super::RenameObject;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rename_object::RenameObject>>(payload: &RenameObject, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(RenameObject { id: payload.id.clone(), new_name: object.name.clone() })]

    })())
}
//#endregion 🔖️Inverse
