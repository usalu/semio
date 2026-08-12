//! ↩️ Inverse for `ScaleObject` — recovers the pre-mutation `scale` from `base` (unit scale when
//! the object had none).
use super::mutation::ScaleObject;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleObject, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::ScaleObject(ScaleObject { pane: payload.pane, object_id: payload.object_id.clone(), new_scale: object.scale.unwrap_or([1.0, 1.0, 1.0]) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
