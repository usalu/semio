//! ↩️ Inverse for `ReplaceObjectGeometry` — recovers the pre-mutation geometry-identity trio.
use super::mutation::ReplaceObjectGeometry;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceObjectGeometry, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| {
            vec![CadMutation::ReplaceObjectGeometry(ReplaceObjectGeometry {
                pane: payload.pane,
                object_id: payload.object_id.clone(),
                new_extent: object.extent,
                new_mesh_url: object.mesh_url.clone(),
                new_solid_handle: object.solid_handle.clone(),
            })]
        })
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
