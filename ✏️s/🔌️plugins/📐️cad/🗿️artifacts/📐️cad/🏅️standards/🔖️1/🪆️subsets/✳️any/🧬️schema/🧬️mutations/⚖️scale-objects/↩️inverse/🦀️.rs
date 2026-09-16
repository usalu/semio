//! ↩️ Inverse for `ScaleObjects` — the same op carrying each touched object's PRE-scale factors.
use super::ScaleObjects;
use crate::mutations::{CadMutation, CadObjectScale};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub const CAD_IDENTITY_SCALE: [f64; 3] = [1.0, 1.0, 1.0];

pub fn inverse(payload: &ScaleObjects, base: &CadSnapshot) -> Vec<CadMutation> {
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return Vec::new();
    };
    let objects = crate::cad_scene_pane_objects(&scene, payload.pane);
    let placements: Vec<CadObjectScale> = payload
        .placements
        .iter()
        .filter_map(|placement| objects.iter().find(|object| object.id == placement.object_id).map(|object| CadObjectScale { object_id: object.id.clone(), new_scale: object.scale.unwrap_or(CAD_IDENTITY_SCALE) }))
        .collect();
    if placements.is_empty() {
        return Vec::new();
    }
    vec![CadMutation::ScaleObjects(ScaleObjects { pane: payload.pane, placements })]
}
//#endregion 🔖️Inverse
