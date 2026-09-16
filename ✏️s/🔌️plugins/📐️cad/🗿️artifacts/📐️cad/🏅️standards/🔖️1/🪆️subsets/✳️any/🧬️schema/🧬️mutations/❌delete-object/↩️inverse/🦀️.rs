//! ↩️ Inverse for `DeleteObject` — recreates the captured object at the exact slot it occupied.
use super::DeleteObject;
use crate::mutations::{cad_object_spec_of, create_object, CadMutation};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteObject, base: &CadSnapshot) -> Vec<CadMutation> {
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return Vec::new();
    };
    let objects = crate::cad_scene_pane_objects(&scene, payload.pane);
    let Some(index) = objects.iter().position(|object| object.id == payload.object_id) else {
        return Vec::new();
    };
    vec![CadMutation::CreateObject(create_object::CreateObject { pane: payload.pane, index: index as u32, object: cad_object_spec_of(&objects[index]) })]
}
//#endregion 🔖️Inverse
