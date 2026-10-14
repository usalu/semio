//! ↩️ `reorder-objects` — undo reorders back to the base-state index; missing id ⇒ `Vec::new()`.

use super::ReorderObjects;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::reorder_objects::ReorderObjects>>(payload: &ReorderObjects, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(original_index) = base.objects.iter().position(|object| object.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ReorderObjects { id: payload.id.clone(), to_index: original_index })]

    })())
}
//#endregion 🔖️Inverse
