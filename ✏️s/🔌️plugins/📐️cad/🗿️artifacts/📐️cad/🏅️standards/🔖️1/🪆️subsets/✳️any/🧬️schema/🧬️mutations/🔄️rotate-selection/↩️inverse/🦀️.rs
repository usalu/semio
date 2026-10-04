//! ↩️ Inverse for `RotateSelection` — ONE absolute `rotate-objects` restoring every BASE orientation the turn changes.
use super::RotateSelection;
use crate::mutations::rotate_objects::{inverse::CAD_IDENTITY_ORIENTATION, RotateObjects};
use crate::mutations::{cad_quaternion_product, cad_selection_inverse_objects, CadMutation, CadObjectOrientation};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RotateSelection, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(delta) = payload.delta() else {
        return Vec::new();
    };
    let placements: Vec<CadObjectOrientation> = cad_selection_inverse_objects(payload.pane, &payload.targets, payload.angle == 0.0, base, |object| {
        let next = cad_quaternion_product(delta, object.orientation.unwrap_or(CAD_IDENTITY_ORIENTATION));
        object.orientation = (next != CAD_IDENTITY_ORIENTATION).then_some(next);
    })
    .into_iter()
    .map(|object| CadObjectOrientation { object_id: object.id, new_orientation: object.orientation.unwrap_or(CAD_IDENTITY_ORIENTATION) })
    .collect();
    if placements.is_empty() {
        return Vec::new();
    }
    vec![CadMutation::RotateObjects(RotateObjects { pane: payload.pane, placements })]

    })())
}
//#endregion 🔖️Inverse
