//! ↩️ Inverse for `RotateObjects` — the negated angle undoes a relative composed rotation.
use super::mutation::RotateObjects;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RotateObjects, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::RotateObjects(RotateObjects { object_ids: payload.object_ids.clone(), ax: payload.ax, ay: payload.ay, az: payload.az, angle: -payload.angle })]
}
//#endregion 🔖️Inverse
