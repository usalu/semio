//! ↩️ Inverse for `MoveObjects` — the same op carrying each touched object's PRE-move origin.
use super::MoveObjects;
use crate::mutations::{CadMutation, CadObjectOrigin};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveObjects, base: &CadSnapshot) -> Vec<CadMutation> {
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return Vec::new();
    };
    let objects = crate::cad_scene_pane_objects(&scene, payload.pane);
    let placements: Vec<CadObjectOrigin> = payload
        .placements
        .iter()
        .filter_map(|placement| objects.iter().find(|object| object.id == placement.object_id).map(|object| CadObjectOrigin { object_id: object.id.clone(), new_origin: object.origin }))
        .collect();
    if placements.is_empty() {
        return Vec::new();
    }
    vec![CadMutation::MoveObjects(MoveObjects { pane: payload.pane, placements })]
}
//#endregion 🔖️Inverse
