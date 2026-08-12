//! ↩️ Inverse for `RotateObject` — recovers the pre-mutation `orientation` from `base` (identity
//! quaternion when the object had none).
use super::mutation::RotateObject;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RotateObject, base: &CadSnapshot) -> Vec<CadMutation> {
    cad_pane_objects(base, payload.pane)
        .iter()
        .find(|object| object.id == payload.object_id)
        .map(|object| vec![CadMutation::RotateObject(RotateObject { pane: payload.pane, object_id: payload.object_id.clone(), new_orientation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
