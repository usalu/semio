//! ↩️ `change-object-smooth-shading` — undo restores the base-state flag; missing id ⇒ `Vec::new()`.

use super::ChangeObjectSmoothShading;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_object_smooth_shading::ChangeObjectSmoothShading>>(payload: &ChangeObjectSmoothShading, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ChangeObjectSmoothShading { id: payload.id.clone(), new_smooth_shading: object.smooth_shading })]

    })())
}
//#endregion 🔖️Inverse
