//! ↩️ Inverse for `RotateObjects` — the same op carrying each touched object's PRE-rotation quaternion.
use super::RotateObjects;
use crate::mutations::{CadMutation, CadObjectOrientation};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub const CAD_IDENTITY_ORIENTATION: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn inverse(payload: &RotateObjects, base: &CadSnapshot) -> Vec<CadMutation> {
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return Vec::new();
    };
    let objects = crate::cad_scene_pane_objects(&scene, payload.pane);
    let placements: Vec<CadObjectOrientation> = payload
        .placements
        .iter()
        .filter_map(|placement| {
            objects
                .iter()
                .find(|object| object.id == placement.object_id)
                .map(|object| CadObjectOrientation { object_id: object.id.clone(), new_orientation: object.orientation.unwrap_or(CAD_IDENTITY_ORIENTATION) })
        })
        .collect();
    if placements.is_empty() {
        return Vec::new();
    }
    vec![CadMutation::RotateObjects(RotateObjects { pane: payload.pane, placements })]
}
//#endregion 🔖️Inverse
