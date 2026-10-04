//! ↩️ Inverse for `ScaleSelection` — ONE absolute `scale-objects` restoring every BASE scale the scaling changes.
use super::ScaleSelection;
use crate::mutations::scale_objects::{inverse::CAD_IDENTITY_SCALE, ScaleObjects};
use crate::mutations::{cad_selection_inverse_objects, CadMutation, CadObjectScale};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleSelection, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let factors = payload.factors;
    let identity = factors == CAD_IDENTITY_SCALE || factors.iter().any(|factor| !(factor.is_finite() && *factor > 0.0));
    let placements: Vec<CadObjectScale> = cad_selection_inverse_objects(payload.pane, &payload.targets, identity, base, |object| {
        let scale = object.scale.unwrap_or(CAD_IDENTITY_SCALE);
        let next = [scale[0] * factors[0], scale[1] * factors[1], scale[2] * factors[2]];
        object.scale = (next != CAD_IDENTITY_SCALE).then_some(next);
    })
    .into_iter()
    .map(|object| CadObjectScale { object_id: object.id, new_scale: object.scale.unwrap_or(CAD_IDENTITY_SCALE) })
    .collect();
    if placements.is_empty() {
        return Vec::new();
    }
    vec![CadMutation::ScaleObjects(ScaleObjects { pane: payload.pane, placements })]

    })())
}
//#endregion 🔖️Inverse
