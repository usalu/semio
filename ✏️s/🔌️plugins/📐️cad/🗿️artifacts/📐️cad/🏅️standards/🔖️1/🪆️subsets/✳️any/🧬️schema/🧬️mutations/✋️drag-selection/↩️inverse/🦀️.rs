//! ↩️ Inverse for `DragSelection` — ONE absolute `move-objects` restoring every BASE origin the drag moves (exact, never
//! a negated offset). Nothing moved ⇒ `Vec::new()`.
use super::DragSelection;
use crate::mutations::{cad_selection_inverse_objects, move_objects::MoveObjects, CadMutation, CadObjectOrigin};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DragSelection, base: &CadSnapshot) -> Vec<CadMutation> {
    let offset = payload.offset;
    let identity = offset == [0.0; 3] || offset.iter().any(|component| !component.is_finite());
    let placements: Vec<CadObjectOrigin> = cad_selection_inverse_objects(payload.pane, &payload.targets, identity, base, |object| object.origin = [object.origin[0] + offset[0], object.origin[1] + offset[1], object.origin[2] + offset[2]])
        .into_iter()
        .map(|object| CadObjectOrigin { object_id: object.id, new_origin: object.origin })
        .collect();
    if placements.is_empty() {
        return Vec::new();
    }
    vec![CadMutation::MoveObjects(MoveObjects { pane: payload.pane, placements })]
}
//#endregion 🔖️Inverse
